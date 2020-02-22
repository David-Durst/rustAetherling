use std::io::Cursor;
use prost::Message;
use std::fmt::Write;
use std::rc::Rc;
use super::proto::{ValueSerialized, TupleValue, SeqValue};
use super::proto::value_serialized::Elems;

/// Convert a buffer with a protobuf representation of a Sequence value
/// to a Rust, Aetherling Sequence value
///
/// # Examples
/// ```
/// use aetherling::languages::sequence::serialize_values::{load_value, save_value, SerializableSeqValue};
/// let saved_value = save_value(&vec!(true,false));
/// let loaded_value = load_value(&saved_value);
///
/// use std::rc::Rc;
/// let mut result_builder: Vec<Rc<String>> = Vec::new();
/// loaded_value.convert_to_flat_atom_list(&mut result_builder, true);
/// let mut test_builder: Vec<Rc<String>> = Vec::new();
/// vec!(true,false).convert_to_flat_atom_list(&mut test_builder, true);
/// assert_eq!(result_builder, test_builder)
/// ```
pub fn load_value<T: AsRef<[u8]>>(src: &T) -> Box<dyn SerializableSeqValue> {
    let serialized_value = ValueSerialized::decode(&mut Cursor::new(src))
        .unwrap();
    deserialize_value(&serialized_value)
}

fn deserialize_value( serialized_value : &ValueSerialized) -> Box<dyn SerializableSeqValue> {
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
                Elems::Seq(e_vec) => {
                    let e_vec_deserialized: Vec<Box<dyn SerializableSeqValue>> =
                        e_vec.values.iter()
                            .map(|e| deserialize_value(e)).collect();
                    Box::new(e_vec_deserialized)
                }
            }
        },
        None => panic!("deserializing empty value")
    }
}
/// Convert a Rust, Aetherling Sequence value to a buffer with a
/// protobuf representation of a Sequence value
///
/// # Examples
/// ```
/// use aetherling::languages::sequence::serialize_values::{load_value, save_value, SerializableSeqValue};
/// let saved_value = save_value(&vec!(1,2));
/// let loaded_value = load_value(&saved_value);
///
/// use std::rc::Rc;
/// let mut result_builder: Vec<Rc<String>> = Vec::new();
/// loaded_value.convert_to_flat_atom_list(&mut result_builder, true);
/// let mut test_builder: Vec<Rc<String>> = Vec::new();
/// vec!(1,2).convert_to_flat_atom_list(&mut test_builder, true);
/// assert_eq!(result_builder, test_builder)
/// ```
pub fn save_value<T: SerializableSeqValue>(src: &T) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.reserve(buffer.encoded_len());
    let proto_value = src.convert_to_rust_proto();
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    proto_value.encode(&mut buffer).unwrap();
    buffer
}

pub trait SerializableSeqValue {
    /// Convert a sequence value to a Rust struct that can be serialized
    /// by protobuf
    fn convert_to_rust_proto(&self) -> ValueSerialized;
    /// Convert a sequence value to a 1D Vec of the atoms' string
    /// representations.
    /// The string vec argument stores the result.
    ///
    /// Call this with an empty `builder` and `top` as True, it will recur and
    /// update those values
    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, top: bool);
}

impl SerializableSeqValue for i32 {
    fn convert_to_rust_proto(&self) -> ValueSerialized {
        ValueSerialized { elems: Some(Elems::Int(*self)) }
    }

    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        match builder.last_mut() {
            Some(s) => write!(Rc::get_mut(s).unwrap(), "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(Rc::from(s));
                write_result
            }
        }.unwrap();
    }
}

impl SerializableSeqValue for bool {
    fn convert_to_rust_proto(&self) -> ValueSerialized {
        ValueSerialized { elems: Some(Elems::Bit(*self)) }
    }

    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        match builder.last_mut() {
            Some(s) => write!(Rc::get_mut(s).unwrap(), "{}", self),
            None => {
                let mut s = String::new();
                let write_result = write!(s, "{}", self);
                builder.push(Rc::from(s));
                write_result
            }
        }.unwrap();
    }
}

impl<A: SerializableSeqValue, B: SerializableSeqValue> SerializableSeqValue for (A, B) {
    fn convert_to_rust_proto(&self) -> ValueSerialized {
        let (a, b) = self;
        let tuple_value = TupleValue {
            left: Some(Box::new(a.convert_to_rust_proto())),
            right: Some(Box::new(b.convert_to_rust_proto()))
        };
        ValueSerialized { elems: Some(Elems::Tuple(Box::new(tuple_value))) }
    }

    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, _: bool) {
        // ensure builder isn't empty
        match builder.last_mut() {
            Some(s) => s,
            None => {
                builder.push(Rc::from(String::new()));
                // the compiler doesn't know this is safe, but I do
                builder.last_mut().unwrap()
            }
        };
        let (a,b) = self;
        // now I know builder isn't empty and I'm trusting
        // that tuples are only of atoms.
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), "(").unwrap();
        a.convert_to_flat_atom_list(builder, false);
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), ",").unwrap();
        b.convert_to_flat_atom_list(builder, false);
        write!(Rc::get_mut(builder.last_mut().unwrap()).unwrap(), ")").unwrap();
    }
}

impl<A: SerializableSeqValue> SerializableSeqValue for Vec<A> {
    fn convert_to_rust_proto(&self) -> ValueSerialized {
        let seq_values_serialized =
            self.iter().map(|e| e.convert_to_rust_proto()).collect();
        ValueSerialized { elems: Some(Elems::Seq(SeqValue { values: seq_values_serialized })) }
    }

    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, top: bool) {
        for (idx, elem) in self.iter().enumerate() {
            // if this is the first element, only add a vec if this is the top vector
            // otherwise on first index let parent vector create string
            // always insert string otherwise
            if (idx == 0 && top) || (idx > 0) {
                builder.push(Rc::from(String::new()))
            }
            elem.convert_to_flat_atom_list(builder, false)
        }
    }
}

impl<A: SerializableSeqValue + ?Sized > SerializableSeqValue for Box<A> {
    fn convert_to_rust_proto(&self) -> ValueSerialized {
        unimplemented!()
    }

    fn convert_to_flat_atom_list(&self, builder: &mut Vec<Rc<String>>, top: bool) {
        self.as_ref().convert_to_flat_atom_list(builder, top)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_flat_atom_list_int() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        1.convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("1"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_bool() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        true.convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("true"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_tuple() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        (3, false).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("(3,false)"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_array() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        vec!(4,2,1,5).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("4")), Rc::from(String::from("2")),
                                 Rc::from(String::from("1")), Rc::from(String::from("5"))))
    }

    #[test]
    fn test_convert_to_flat_atom_list_nested_array() {
        let mut builder: Vec<Rc<String>> = Vec::new();
        vec!(vec!(4,2),vec!(1,5)).convert_to_flat_atom_list(&mut builder, true);
        assert_eq!(builder, vec!(Rc::from(String::from("4")), Rc::from(String::from("2")),
                                 Rc::from(String::from("1")), Rc::from(String::from("5"))))
    }
}

