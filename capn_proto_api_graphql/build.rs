extern crate capnpc;

fn main() {
    ::capnpc::CompilerCommand::new()
        .output_path("src")
        .src_prefix("capnp")
        .file("capnp/service.capnp")
        .run()
        .expect("failed to compile schema");
}
