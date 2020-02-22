use aetherling::{run, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 5 as first argument is path to executable
    if args.len() != 5 {
        panic!("must have 4 arguments");
    }

    let conf = Config {
        sequence_values_proto_path: args[1].clone(),
        space_time_type_proto_path: args[2].clone(),
        output_values_csv_path: args[3].clone(),
        output_valids_csv_path: args[4].clone()
    };

    run(conf);
}
