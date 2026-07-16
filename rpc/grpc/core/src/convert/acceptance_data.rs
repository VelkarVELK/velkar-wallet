use crate::protowire::{self};
use crate::{from, try_from};
use velkar_rpc_core::RpcError;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: &velkar_rpc_core::RpcDataVerbosityLevel, protowire::RpcDataVerbosityLevel, {
    match item {
        velkar_rpc_core::RpcDataVerbosityLevel::None => protowire::RpcDataVerbosityLevel::None,
        velkar_rpc_core::RpcDataVerbosityLevel::Low => protowire::RpcDataVerbosityLevel::Low,
        velkar_rpc_core::RpcDataVerbosityLevel::High => protowire::RpcDataVerbosityLevel::High,
        velkar_rpc_core::RpcDataVerbosityLevel::Full => protowire::RpcDataVerbosityLevel::Full,
    }
});

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

try_from!(item: &protowire::RpcDataVerbosityLevel, velkar_rpc_core::RpcDataVerbosityLevel,  {
    match item {
        protowire::RpcDataVerbosityLevel::None => velkar_rpc_core::RpcDataVerbosityLevel::None,
        protowire::RpcDataVerbosityLevel::Low => velkar_rpc_core::RpcDataVerbosityLevel::Low,
        protowire::RpcDataVerbosityLevel::High => velkar_rpc_core::RpcDataVerbosityLevel::High,
        protowire::RpcDataVerbosityLevel::Full => velkar_rpc_core::RpcDataVerbosityLevel::Full
    }
});
