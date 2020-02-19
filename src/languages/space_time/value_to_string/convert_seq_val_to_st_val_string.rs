use super::to_atom_strings::ToAtomStrings;
use super::super::types::Type;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::io::Write;

pub fn convert_seq_val_to_st_val_string<T: ToAtomStrings, W: Write>(seq_val: T, st_type: Type, sink: &mut W) -> Result<(), Box<dyn Error>> {
    let mut flat_val_strs: Vec<Rc<String>> = Vec::new();
    seq_val.convert_to_flat_atom_list(&mut flat_val_strs, true);
    let mut flat_val_idx_to_str: HashMap<usize, Rc<String>> = flat_val_strs.into_iter().enumerate().collect();

    let total_width = st_type.atoms_per_valid();
    let total_time = st_type.clocks();
    let st_vals = convert_seq_idxs_to_vals_to_time_space_vec(&mut flat_val_idx_to_str, st_type);

    // write a csv array where only wrap the space dimension if it has more than 1 element
    sink.write("[".as_ref())?;
    for t in 0..total_time {
        if t > 0 {
            sink.write(",".as_ref())?;
        }
        if total_width == 1 {
            sink.write(st_vals[t as usize][0].as_ref().as_bytes())?;
        }
        else {
            sink.write("[".as_ref())?;
            for s in 0..total_width {
                sink.write(st_vals[t as usize][s as usize].as_ref().as_bytes())?;
                if s < total_width - 1 {
                    sink.write(",".as_ref())?;
                }
            }
            sink.write("]".as_ref())?;
        }
    }
    sink.write("]".as_ref())?;
    sink.flush()?;
    Ok(())
}

fn convert_seq_idxs_to_vals_to_time_space_vec(seq_idxs_to_vals: &mut HashMap<usize, Rc<String>>,
                                              st_type: Type) -> Vec<Vec<Rc<String>>> {
    let total_width = st_type.atoms_per_valid();
    let total_time = st_type.clocks();
    let valid_time = st_type.valid_clocks();
    let mut time_space_values_vec: Vec<Vec<Rc<String>>> = Vec::with_capacity(total_time as usize);
    let def_str = Rc::new(String::from("0"));
    for _ in 0..total_time {
        let mut inner_vec = Vec::with_capacity(total_width as usize);
        for _ in 0..total_width {
           inner_vec.push(def_str.clone()) ;
        }
        time_space_values_vec.push(inner_vec)
    }
    set_val_in_time_space_vec(seq_idxs_to_vals, &mut time_space_values_vec,
                              &st_type, total_width, total_time, valid_time, 0, 0, true, 0);
    time_space_values_vec
}

fn set_val_in_time_space_vec(seq_idx_to_vals: &mut HashMap<usize, Rc<String>>,
                             time_space_values_vec: &mut Vec<Vec<Rc<String>>>,
                             st_type: &Type, total_width: u32, total_time: u32,
                             valid_time: u32, cur_space: u32, cur_time: u32,
                             valid: bool, cur_idx: u32) {
    match st_type {
        Type::STuple { n, elem_type } => {
            let element_width = total_width / *n;
            let element_time = total_time;
            let element_valid_time = valid_time;
            for i in 0..=n - 1 {
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, elem_type,
                                          element_width, element_time,
                                          element_valid_time, cur_space + i * element_width,
                                          cur_time, valid,
                                          cur_idx + i * element_width * element_valid_time)
            }
        }
        Type::SSeq { n, elem_type} => {
            let element_width = total_width / *n;
            let element_time = total_time;
            let element_valid_time = valid_time;
            for i in 0..=n - 1 {
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, elem_type,
                                          element_width, element_time,
                                          element_valid_time, cur_space + i * element_width,
                                          cur_time, valid,
                                          cur_idx + i * element_width * element_valid_time)
            }
        }
        Type::TSeq {n, i, elem_type} => {
            let element_width = total_width;
            let element_time = total_time / (*n + *i);
            let element_valid_time = valid_time / *n;
            for i in 0..=n - 1 {
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, elem_type,
                                          element_width, element_time,
                                          element_valid_time, cur_space,
                                          cur_time + i * element_time, valid && i < *n,
                                          cur_idx + i * element_width * element_valid_time)
            }
        }
        _ =>  {
            if valid {
                time_space_values_vec[cur_time as usize][cur_space as usize] =
                    seq_idx_to_vals.get_mut(&(cur_idx as usize)).unwrap().clone();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_seq_val_to_st_val_string_sseq_4_int() {
        let mut builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4),
                                         Type::SSeq {n: 4, elem_type: Box::from(Type::Int)},
                                         &mut builder).unwrap();
        let data = String::from_utf8(builder).unwrap();
        assert_eq!(data, String::from("[[1,3,2,4]]"))
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_4_int() {
        let mut builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4),
                                         Type::TSeq {n: 4, i: 0, elem_type: Box::from(Type::Int)},
                                         &mut builder).unwrap();
        let data = String::from_utf8(builder).unwrap();
        assert_eq!(data, String::from("[1,3,2,4]"))
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_2_1_int() {
        let mut builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3),
                                         Type::TSeq {n: 2, i: 1, elem_type: Box::from(Type::Int)},
                                         &mut builder).unwrap();
        let data = String::from_utf8(builder).unwrap();
        assert_eq!(data, String::from("[1,3,0]"))
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_3_0_sseq_2_int() {
        let mut builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4,6,5),
                                         Type::TSeq {n: 3, i: 0, elem_type: Box::from(
                                             Type::SSeq {n: 2, elem_type: Box::from(Type::Int)})},
                                         &mut builder).unwrap();
        let data = String::from_utf8(builder).unwrap();
        assert_eq!(data, String::from("[[1,3],[2,4],[6,5]]"))
    }
/*
    #[test]
    fn test_convert_seq_val_to_st_val_string_big() {
        let mut builder = Vec::new();
        let seq_val: Vec<i32> = (1..=1920*1080).collect();
        convert_seq_val_to_st_val_string(seq_val,
                                         Type::TSeq {n: (1920 * 1080 / 2), i: 0, elem_type: Box::from(
                                             Type::SSeq {n: 2, elem_type: Box::from(Type::Int)})},
                                         &mut builder).unwrap();
        let data = String::from_utf8(builder).unwrap();
        assert_eq!(17551297, data.len())
    }
    */
}
