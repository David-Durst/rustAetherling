fn main() {
    prost_build::compile_protos(&["src/languages/space_time/types.proto"],
                                &["src/languages/space_time"]).unwrap();
}