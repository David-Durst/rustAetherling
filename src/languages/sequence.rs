pub mod types;
pub mod serialize;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/languages.sequence.proto.rs"));
}
