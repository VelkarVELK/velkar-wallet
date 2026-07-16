use crate::pb::velkard_message::Payload as VelkardMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum VelkardMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
    BlockBody,
    RequestBlockBodies,
}

impl From<&VelkardMessagePayload> for VelkardMessagePayloadType {
    fn from(payload: &VelkardMessagePayload) -> Self {
        match payload {
            VelkardMessagePayload::Addresses(_) => VelkardMessagePayloadType::Addresses,
            VelkardMessagePayload::Block(_) => VelkardMessagePayloadType::Block,
            VelkardMessagePayload::Transaction(_) => VelkardMessagePayloadType::Transaction,
            VelkardMessagePayload::BlockLocator(_) => VelkardMessagePayloadType::BlockLocator,
            VelkardMessagePayload::RequestAddresses(_) => VelkardMessagePayloadType::RequestAddresses,
            VelkardMessagePayload::RequestRelayBlocks(_) => VelkardMessagePayloadType::RequestRelayBlocks,
            VelkardMessagePayload::RequestTransactions(_) => VelkardMessagePayloadType::RequestTransactions,
            VelkardMessagePayload::IbdBlock(_) => VelkardMessagePayloadType::IbdBlock,
            VelkardMessagePayload::InvRelayBlock(_) => VelkardMessagePayloadType::InvRelayBlock,
            VelkardMessagePayload::InvTransactions(_) => VelkardMessagePayloadType::InvTransactions,
            VelkardMessagePayload::Ping(_) => VelkardMessagePayloadType::Ping,
            VelkardMessagePayload::Pong(_) => VelkardMessagePayloadType::Pong,
            VelkardMessagePayload::Verack(_) => VelkardMessagePayloadType::Verack,
            VelkardMessagePayload::Version(_) => VelkardMessagePayloadType::Version,
            VelkardMessagePayload::TransactionNotFound(_) => VelkardMessagePayloadType::TransactionNotFound,
            VelkardMessagePayload::Reject(_) => VelkardMessagePayloadType::Reject,
            VelkardMessagePayload::PruningPointUtxoSetChunk(_) => VelkardMessagePayloadType::PruningPointUtxoSetChunk,
            VelkardMessagePayload::RequestIbdBlocks(_) => VelkardMessagePayloadType::RequestIbdBlocks,
            VelkardMessagePayload::UnexpectedPruningPoint(_) => VelkardMessagePayloadType::UnexpectedPruningPoint,
            VelkardMessagePayload::IbdBlockLocator(_) => VelkardMessagePayloadType::IbdBlockLocator,
            VelkardMessagePayload::IbdBlockLocatorHighestHash(_) => VelkardMessagePayloadType::IbdBlockLocatorHighestHash,
            VelkardMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                VelkardMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            VelkardMessagePayload::DonePruningPointUtxoSetChunks(_) => VelkardMessagePayloadType::DonePruningPointUtxoSetChunks,
            VelkardMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                VelkardMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            VelkardMessagePayload::BlockWithTrustedData(_) => VelkardMessagePayloadType::BlockWithTrustedData,
            VelkardMessagePayload::DoneBlocksWithTrustedData(_) => VelkardMessagePayloadType::DoneBlocksWithTrustedData,
            VelkardMessagePayload::RequestPruningPointAndItsAnticone(_) => VelkardMessagePayloadType::RequestPruningPointAndItsAnticone,
            VelkardMessagePayload::BlockHeaders(_) => VelkardMessagePayloadType::BlockHeaders,
            VelkardMessagePayload::RequestNextHeaders(_) => VelkardMessagePayloadType::RequestNextHeaders,
            VelkardMessagePayload::DoneHeaders(_) => VelkardMessagePayloadType::DoneHeaders,
            VelkardMessagePayload::RequestPruningPointUtxoSet(_) => VelkardMessagePayloadType::RequestPruningPointUtxoSet,
            VelkardMessagePayload::RequestHeaders(_) => VelkardMessagePayloadType::RequestHeaders,
            VelkardMessagePayload::RequestBlockLocator(_) => VelkardMessagePayloadType::RequestBlockLocator,
            VelkardMessagePayload::PruningPoints(_) => VelkardMessagePayloadType::PruningPoints,
            VelkardMessagePayload::RequestPruningPointProof(_) => VelkardMessagePayloadType::RequestPruningPointProof,
            VelkardMessagePayload::PruningPointProof(_) => VelkardMessagePayloadType::PruningPointProof,
            VelkardMessagePayload::Ready(_) => VelkardMessagePayloadType::Ready,
            VelkardMessagePayload::BlockWithTrustedDataV4(_) => VelkardMessagePayloadType::BlockWithTrustedDataV4,
            VelkardMessagePayload::TrustedData(_) => VelkardMessagePayloadType::TrustedData,
            VelkardMessagePayload::RequestIbdChainBlockLocator(_) => VelkardMessagePayloadType::RequestIbdChainBlockLocator,
            VelkardMessagePayload::IbdChainBlockLocator(_) => VelkardMessagePayloadType::IbdChainBlockLocator,
            VelkardMessagePayload::RequestAntipast(_) => VelkardMessagePayloadType::RequestAntipast,
            VelkardMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                VelkardMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
            VelkardMessagePayload::BlockBody(_) => VelkardMessagePayloadType::BlockBody,
            VelkardMessagePayload::RequestBlockBodies(_) => VelkardMessagePayloadType::RequestBlockBodies,
        }
    }
}
