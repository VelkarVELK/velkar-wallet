fn main() {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(false)
        .compile_protos(&["proto/velkarwalletd.proto"], &["proto"])
        .expect("failed to compile velkarwalletd proto");
}
