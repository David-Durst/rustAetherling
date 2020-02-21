fn main() {
    prost_build::compile_protos(&["protoAetherling/spacetime.proto"],
                                &["protoAetherling"]).unwrap();
}