use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use velkar_grpc_core::{
    ops::VelkardPayloadOps,
    protowire::{VelkardRequest, VelkardResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type VelkardMethod = Method<ServerContext, Connection, VelkardRequest, VelkardResponse>;
pub type DynVelkardMethod = Arc<dyn MethodTrait<ServerContext, Connection, VelkardRequest, VelkardResponse>>;
pub type VelkardDropFn = DropFn<VelkardRequest, VelkardResponse>;
pub type VelkardRoutingPolicy = RoutingPolicy<VelkardRequest, VelkardResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`VelkardPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<VelkardPayloadOps, DynVelkardMethod>,
    method_not_implemented: DynVelkardMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, velkard_request: VelkardRequest| {
            Box::pin(async move {
                match velkard_request.payload {
                    Some(ref request) => Ok(VelkardResponse {
                        id: velkard_request.id,
                        payload: Some(VelkardPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: VelkardPayloadOps, method: VelkardMethod) {
        let method: DynVelkardMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: VelkardPayloadOps, method: VelkardMethod) {
        let method: DynVelkardMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: VelkardPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: VelkardRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, VelkardRequest, VelkardResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, VelkardRequest, VelkardResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &VelkardPayloadOps,
        connection: Connection,
        request: VelkardRequest,
    ) -> GrpcServerResult<VelkardResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &VelkardPayloadOps) -> DynVelkardMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
