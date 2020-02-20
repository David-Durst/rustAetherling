fn main() {
    prost_build::compile_protos(&["protoAetherling/space_time.proto"],
                                &["protoAetherling"]).unwrap();
}