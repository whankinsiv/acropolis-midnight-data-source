fn main() {
    tonic_build::configure()
        .compile_protos(&["proto/midnight_indexer.proto"], &["proto"])
        .unwrap();
}
