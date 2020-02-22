pub mod languages;
use languages::space_time::serialize;
use languages::sequence::serialize_values;
use languages::seq_value_to_st_value_and_valid_strings;
use std::fs;
use std::fs::File;

pub fn run(conf: Config) {
    let seq_values_proto = fs::read_to_string(conf.sequence_values_proto_path)
        .expect("couldn't read seq values proto file");
    let st_type_proto = fs::read_to_string(conf.space_time_type_proto_path)
        .expect("couldn't read space time type proto file");
    let seq_values = serialize_values::load_value(&seq_values_proto);
    let st_type = serialize::load_type(&st_type_proto);
    let mut output_values_file = File::create(conf.output_values_csv_path).unwrap();
    let mut output_valids_file = File::create(conf.output_valids_csv_path).unwrap();
    seq_value_to_st_value_and_valid_strings::convert_seq_val_to_st_val_and_valid_strings(
        seq_values, st_type, &mut output_values_file, &mut output_valids_file
    ).unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub sequence_values_proto_path: String,
    pub space_time_type_proto_path: String,
    pub output_values_csv_path: String,
    pub output_valids_csv_path: String
}
