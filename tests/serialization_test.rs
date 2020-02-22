use std::fs;
use std::rc::Rc;
use aetherling::languages::sequence::serialize_values as seq_val_ser;
use aetherling::languages::space_time::serialize as st_ser;
use aetherling::languages::space_time::types::Type;
use aetherling::languages::sequence::serialize_values::SerializableSeqValue;
use std::path::PathBuf;

#[test]
fn deserialize_haskell_seq_value() {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/tests/seq_value_proto.bin");
    let seq_value_proto = fs::read(file)
        .expect("couldn't read sequence value proto file");
    let seq_value = seq_val_ser::load_value(&seq_value_proto);
    let mut result_builder: Vec<Rc<String>> = Vec::new();
    seq_value.convert_to_flat_atom_list(&mut result_builder, true);
    let mut test_builder: Vec<Rc<String>> = Vec::new();
    vec!(0,-1,2,3).convert_to_flat_atom_list(&mut test_builder, true);
    assert_eq!(result_builder, test_builder)
}

#[test]
fn deserialize_haskell_st_type() {
    let mut file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file.push("resources/tests/st_type_proto.bin");
    let st_type_proto = fs::read_to_string("/tmp/ae_proto_types_05225-147.bin")
        .expect("couldn't read space time type proto file");
    let st_type = st_ser::load_type(&st_type_proto);
    assert_eq!(st_type, Type::TSeq{ n: 4, i: 0, elem_type: Box::new(Type::Int) });
}
