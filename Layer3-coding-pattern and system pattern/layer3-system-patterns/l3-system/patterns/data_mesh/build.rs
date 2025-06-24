fn main() {
    prost_build::compile_protos(&["src/user.proto"], &["src"])
        .expect("compile protobuf");
}
