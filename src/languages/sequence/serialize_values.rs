use super::types::Type;
use std::io::Cursor;
use prost::Message;
use super::proto::{ValueSerialized, TupleValue, SeqValue};
use super::proto::value_serialized::Elems;
use super::super::value_to_string::to_atom_strings::ToAtomStrings;

/// Convert a buffer with a protobuf representation of a Sequence value
/// to a Rust, Aetherling Sequence value
///
/// # Examples
/// ```
/// use aetherling::languages::sequence::serialize_types::{ load_type, save_type };
/// use aetherling::languages::sequence::types::Type;
/// let saved_typed = save_type(&Type::Int);
/// let loaded_type = load_type(&saved_typed);
///
/// assert_eq!(loaded_type, Type::Int)
/// ```
pub fn load_value<T: AsRef<[u8]>>(src: &T) -> Box<dyn ToAtomStrings> {
    let serialized_value = ValueSerialized::decode(&mut Cursor::new(src))
        .unwrap();
    deserialize_value(&serialized_value)
}

fn deserialize_value( serialized_value : &ValueSerialized) -> Box<dyn ToAtomStrings> {
    match &serialized_value.elems {
        Some(elem) => {
            match elem {
                Elems::Int(e) => Box::new(*e),
                Elems::Bit(e) => Box::new(*e),
                Elems::Tuple(e_box) => {
                    let e_left = match &e_box.left {
                        Some(e_left_elem) => deserialize_value(&e_left_elem),
                        None => panic!("tuple with no left element")
                    };
                    let e_right = match &e_box.right {
                        Some(e_right_elem) => deserialize_value(&e_right_elem),
                        None => panic!("tuple with no right element")
                    };
                    Box::new((e_left, e_right))
                }
                _ => unimplemented!("hi")
            }
        },
        None => panic!("deserializing empty value")
    }
    /*
    // can't convert int to enum in match statement easily when using prost
    if *v == TypeVersion::Unit as i32 {
        Type::Unit
    } else if *v == TypeVersion::Bit as i32 {
        Type::Bit
    } else if *v == TypeVersion::Int as i32 {
        Type::Int
    } else if *v == TypeVersion::ATuple as i32 {
        let left = deserialize_type(&children[0]);
        let right = deserialize_type(&children[1]);
        Type::ATuple { left: Box::new(left), right: Box::new(right) }
    } else {
        let elem_type = deserialize_type(&children[0]);
        Type::Seq { n: *n, elem_type: Box::new(elem_type) }
    }
    */
}

/*

/// Convert a Rust, Aetherling Sequence type to a buffer with a
/// protobuf representation of a Sequence type
///
/// # Examples
/// ```
/// use aetherling::languages::sequence::serialize_types::{ load_type, save_type };
/// use aetherling::languages::sequence::types::Type;
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
            TypeSerialized {v: TypeVersion::Unit as i32, n: 0, children: Vec::new()},
        Type::Bit =>
            TypeSerialized {v: TypeVersion::Bit as i32, n: 0, children: Vec::new()},
        Type::Int =>
            TypeSerialized {v: TypeVersion::Int as i32, n: 0, children: Vec::new()},
        Type::ATuple { .. } =>
            TypeSerialized {v: TypeVersion::ATuple as i32, n: 0, children: Vec::new()},
        Type::Seq { n, elem_type} => {
            let mut children = Vec::new();
            children.push(serialize_type(elem_type));
            TypeSerialized {v: TypeVersion::Seq as i32, n: *n, children}
        }
    }
}
*/
