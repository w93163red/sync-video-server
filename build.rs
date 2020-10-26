fn main() {
    prost_build::compile_protos(&["protobuf/message.proto"],
                                &["src/"]).unwrap();
}