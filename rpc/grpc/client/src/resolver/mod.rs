use super::error::Result;
use core::fmt::Debug;
use velkar_grpc_core::{
    ops::VelkardPayloadOps,
    protowire::{VelkardRequest, VelkardResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: VelkardPayloadOps, request: &VelkardRequest) -> VelkardResponseReceiver;
    fn handle_response(&self, response: VelkardResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type VelkardResponseSender = oneshot::Sender<Result<VelkardResponse>>;
pub(crate) type VelkardResponseReceiver = oneshot::Receiver<Result<VelkardResponse>>;
