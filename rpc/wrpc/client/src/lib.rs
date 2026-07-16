//!
//! # wRPC Client for Rusty Velkar p2p Node
//!
//! This crate provides a WebSocket RPC client for Rusty Velkar p2p node. It is based on the
//! [wRPC](https://docs.rs/workflow-rpc) crate that offers WebSocket RPC implementation
//! for Rust based on Borsh and Serde JSON serialization. wRPC is a lightweight RPC framework
//! meant to function as an IPC (Inter-Process Communication) mechanism for Rust applications.
//!
//! Rust examples on using wRPC client can be found in the
//! [examples](https://github.com/velkarnet/velkar/tree/master/rpc/wrpc/examples) folder.
//!
//! WASM bindings for wRPC client can be found in the [`velkar-wrpc-wasm`](https://docs.rs/velkar-wrpc-wasm) crate.
//!
//! The main struct managing Velkar RPC client connections is the [`VelkarRpcClient`].
//!

pub mod client;
pub mod error;
mod imports;
pub mod result;
pub use imports::{VelkarRpcClient, Resolver, WrpcEncoding};
pub mod node;
pub mod parse;
pub mod prelude;
pub mod resolver;
