
pub mod ast;
pub mod types;
pub mod value_to_string;

pub mod types_serialized {
    include!(concat!(env!("OUT_DIR"), "/languages.space_time.types_serialized.rs"));
}

use std::io::Read;
use std::io::Cursor;
use types::Type;
use prost::Message;
pub fn load_type<T: Read + AsRef<[u8]>>(src: &mut T) -> Type {
    let derialized_type = types_serialized::TypeSerialized::decode(&mut Cursor::new(src))
        .unwrap();
    println!("The loaded struct is: {:?}", derialized_type);
    unimplemented!()
}
