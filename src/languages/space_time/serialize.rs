use super::types::Type;
use std::io::Cursor;
use prost::Message;
use super::proto::{TypeSerialized, TypeVersion};

/// Convert a buffer with a protobuf representation of a Space-Time type
/// to a Rust, Aetherling Space-Time type
///
/// # Examples
/// ```
/// use aetherling::languages::space_time::serialize::{ load_type, save_type };
/// use aetherling::languages::space_time::types::Type;
/// let saved_typed = save_type(&Type::UInt8);
/// let loaded_type = load_type(&saved_typed);
///
/// assert_eq!(loaded_type, Type::UInt8)
/// ```
pub fn load_type<T: AsRef<[u8]>>(src: &T) -> Type {
    let serialized_type = TypeSerialized::decode(&mut Cursor::new(src))
        .unwrap();
    deserialize_type(&serialized_type)
}

fn deserialize_type(TypeSerialized {v, n, i, children} : &TypeSerialized) -> Type {
    // can't convert int to enum in match statement easily when using prost
    if *v == TypeVersion::Unit as i32 {
        Type::Unit
    } else if *v == TypeVersion::Bit as i32 {
        Type::Bit
    } else if *v == TypeVersion::Int8 as i32 {
        Type::Int8
    } else if *v == TypeVersion::UInt8 as i32 {
        Type::UInt8
    } else if *v == TypeVersion::Int16 as i32 {
        Type::Int16
    } else if *v == TypeVersion::UInt16 as i32 {
        Type::UInt16
    } else if *v == TypeVersion::Int32 as i32 {
        Type::Int32
    } else if *v == TypeVersion::UInt32 as i32 {
        Type::UInt32
    } else if *v == TypeVersion::ATuple as i32 {
        let left = deserialize_type(&children[0]);
        let right = deserialize_type(&children[1]);
        Type::ATuple { left: Box::new(left), right: Box::new(right) }
    } else if *v == TypeVersion::STuple as i32 {
        let elem_type = deserialize_type(&children[0]);
        Type::STuple { n: *n, elem_type: Box::new(elem_type) }
    } else if *v == TypeVersion::SSeq as i32 {
        let elem_type = deserialize_type(&children[0]);
        Type::SSeq { n: *n, elem_type: Box::new(elem_type) }
    } else {
        let elem_type = deserialize_type(&children[0]);
        Type::TSeq { n: *n, i: *i, elem_type: Box::new(elem_type) }
    }
}

/// Convert a Rust, Aetherling Space-Time type to a buffer with a
/// protobuf representation of a Space-Time type
///
/// # Examples
/// ```
/// use aetherling::languages::space_time::serialize::{ load_type, save_type };
/// use aetherling::languages::space_time::types::Type;
/// let saved_typed = save_type(&Type::Bit);
/// let loaded_type = load_type(&saved_typed);
///
/// assert_eq!(loaded_type, Type::Bit)
/// ```
pub fn save_type(t: &Type) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.reserve(buffer.encoded_len());
    let proto_type = serialize_type(t);
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    proto_type.encode(&mut buffer).unwrap();
    buffer
}

fn serialize_type(t: &Type) -> TypeSerialized {
    match t {
        Type::Unit =>
            TypeSerialized {v: TypeVersion::Unit as i32, n: 0, i: 0, children: Vec::new()},
        Type::Bit =>
            TypeSerialized {v: TypeVersion::Bit as i32, n: 0, i: 0, children: Vec::new()},
        Type::Int8 =>
            TypeSerialized {v: TypeVersion::Int8 as i32, n: 0, i: 0, children: Vec::new()},
        Type::UInt8 =>
            TypeSerialized {v: TypeVersion::UInt8 as i32, n: 0, i: 0, children: Vec::new()},
        Type::Int16 =>
            TypeSerialized {v: TypeVersion::Int16 as i32, n: 0, i: 0, children: Vec::new()},
        Type::UInt16 =>
            TypeSerialized {v: TypeVersion::UInt16 as i32, n: 0, i: 0, children: Vec::new()},
        Type::Int32 =>
            TypeSerialized {v: TypeVersion::Int32 as i32, n: 0, i: 0, children: Vec::new()},
        Type::UInt32 =>
            TypeSerialized {v: TypeVersion::UInt32 as i32, n: 0, i: 0, children: Vec::new()},
        Type::ATuple { .. } =>
            TypeSerialized {v: TypeVersion::ATuple as i32, n: 0, i: 0, children: Vec::new()},
        Type::STuple { n, elem_type} => {
            let mut children = Vec::new();
            children.push(serialize_type(elem_type));
            TypeSerialized {v: TypeVersion::STuple as i32, n: *n, i: 0, children}
        }
        Type::SSeq { n, elem_type} => {
            let mut children = Vec::new();
            children.push(serialize_type(elem_type));
            TypeSerialized {v: TypeVersion::SSeq as i32, n: *n, i: 0, children}
        }
        Type::TSeq { n, i, elem_type} => {
            let mut children = Vec::new();
            children.push(serialize_type(elem_type));
            TypeSerialized {v: TypeVersion::SSeq as i32, n: *n, i: *i, children}
        }
    }
}
