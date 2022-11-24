fn main() {
    thrift_build::generate_rust("idl/commands.thrift").expect("Failed to generate Rust bindings")
}
