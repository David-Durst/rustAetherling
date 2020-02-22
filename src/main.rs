use aetherling::{run, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        panic!("must have 4 arguments");
    }

    let conf = Config {
        sequence_values_proto_path: args[0].clone(),
        space_time_type_proto_path: args[1].clone(),
        output_values_csv_path: args[2].clone(),
        output_valids_csv_path: args[3].clone()
    };

    run(conf);
}
