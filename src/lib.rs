pub mod languages;
use languages::space_time::serialize;
use languages::sequence::serialize_values;
use languages::seq_value_to_st_value_and_valid_strings;
use std::fs;
use std::fs::File;
use cute::c;

pub fn run(conf: Config) {
    let seq_values_proto = fs::read(conf.sequence_values_proto_path)
        .expect("couldn't read seq values proto file");
    let st_type_proto = fs::read(conf.space_time_type_proto_path)
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

fn get_input(inputs_2d: &Vec<i32>, row_size: i32, r: i32, c: i32) -> i32 {
    if r < 0 || c < 0 {
        253
    } else {
        *inputs_2d.get((r * row_size + c) as usize).unwrap()
    }
}
pub fn stencil_generator(row_size: i32, inputs_2d: &Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let col_size = inputs_2d.len() as i32 / row_size;
    let num_rows = col_size;
    let num_cols = row_size;
    /*
    let mut result = Box::new(Vec::new());
    for r in 0 .. num_rows {
       for c in 0 .. num_cols {
           for stencil_r in [2,1,0] {
               for stencil_c in [2,1,0] {
                   get_input(r,c)
               }
           }
       }
    };
    */
    c![
        c![
            c![
                get_input(inputs_2d, row_size, r - stencil_r, c - stencil_c),
                for stencil_c in [2,1,0].iter()
            ], for stencil_r in [2,1,0].iter()
        ], for r in 0..num_rows, for c in 0..num_cols
    ]
}

pub fn conv_generator(stencil_2d_output: Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    let hask_kernel = [1,2,1,2,4,2,1,2,1];
    let mut result: Vec<i32> = Vec::new();
    for window in stencil_2d_output {
        let flat_window: Vec<i32> = window.concat();
        if flat_window.contains(&253) {
            result.push(253);
        }
        else {
            let mut mac = 0;
            for i in 0..9 {
                mac += hask_kernel[i] * flat_window.get(i).unwrap();
            }
            result.push(mac);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_generator() {
        let input_data = 0..1920*1080;
        println!("length: {}", conv_generator(stencil_generator(1920, &input_data.collect())).len())
    }
}
/*
input_data = [i for i in range(1920*1080)]

def stencil_generator(row_size, inputs_2d):
  col_size = len(inputs_2d) // row_size
  num_rows = col_size
  num_cols = row_size
  def get_input(r, c):
      if ((r < 0) or (c < 0)):
        return 253
      else:
        return inputs_2d[r * row_size + c]
  return [
      [
          [
              get_input(r - stencil_r, c - stencil_c)
              for stencil_c in [2,1,0]
          ] for stencil_r in [2,1,0]
      ] for r in range(num_rows) for c in range(num_cols)
  ]

hask_kernel = [1,2,1,2,4,2,1,2,1]
def conv_generator(stencil_2d_output):
    result = []
    for window in stencil_2d_output:
        flat_window = window[0] + window[1] + window[2]
        if 253 in flat_window:
            result.append(253)
        else:
            mac = 0
            for i in range(9):
                mac += hask_kernel[i] * flat_window[i]
            result.append(mac % 256 // 16)
    return result


print(str(sum(conv_generator(stencil_generator(1920, [i for i in range(1920*1080)])))))
"
*/