fn main() {
    prost_build::compile_protos(&["protobuf/space_time.proto"],
                                &["protobuf"]).unwrap();
}