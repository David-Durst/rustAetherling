use super::to_atom_strings::ToAtomStrings;
use super::super::types::Type;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::io::Write;

pub fn convert_seq_val_to_st_val_string<T: ToAtomStrings, W: Write>(
    seq_val: T, st_type: Type, vals_sink: &mut W, valids_sink: &mut W) -> Result<(), Box<dyn Error>> {
    let mut flat_val_strs: Vec<Rc<String>> = Vec::new();
    seq_val.convert_to_flat_atom_list(&mut flat_val_strs, true);
    let mut flat_val_idx_to_str: HashMap<usize, Rc<String>> = flat_val_strs.into_iter().enumerate().collect();

    let total_width = st_type.atoms_per_valid();
    let total_time = st_type.clocks();
    let mut st_vals: Vec<Vec<Rc<String>>> = Vec::with_capacity(total_time as usize);
    let mut st_valids: Vec<bool> = Vec::with_capacity(total_time as usize);
    convert_seq_idxs_to_vals_to_time_space_vec(&mut flat_val_idx_to_str, &mut st_vals,
                                               &mut st_valids, st_type);

    // write a csv array where only wrap the space dimension if it has more than 1 element
    vals_sink.write("[".as_ref())?;
    valids_sink.write("[".as_ref())?;
    for t in 0..total_time {
        if t > 0 {
            vals_sink.write(",".as_ref())?;
            valids_sink.write(",".as_ref())?;
        }
        if total_width == 1 {
            vals_sink.write(st_vals[t as usize][0].as_ref().as_bytes())?;
        }
        else {
            vals_sink.write("[".as_ref())?;
            for s in 0..total_width {
                vals_sink.write(st_vals[t as usize][s as usize].as_ref().as_bytes())?;
                if s < total_width - 1 {
                    vals_sink.write(",".as_ref())?;
                }
            }
            vals_sink.write("]".as_ref())?;
        }
        valids_sink.write(st_valids[t as usize].to_string().as_ref())?;
    }
    vals_sink.write("]".as_ref())?;
    valids_sink.write("]".as_ref())?;
    vals_sink.flush()?;
    valids_sink.flush()?;
    Ok(())
}

fn convert_seq_idxs_to_vals_to_time_space_vec(seq_idxs_to_vals: &mut HashMap<usize, Rc<String>>,
                                              time_space_values_vec: &mut Vec<Vec<Rc<String>>>,
                                              time_valids_vec: &mut Vec<bool>,
                                              st_type: Type) {
    let total_width = st_type.atoms_per_valid();
    let total_time = st_type.clocks();
    let valid_time = st_type.valid_clocks();
    let def_str = Rc::new(String::from("0"));
    for _ in 0..total_time {
        let mut inner_vec = Vec::with_capacity(total_width as usize);
        for _ in 0..total_width {
           inner_vec.push(def_str.clone());
        }
        time_space_values_vec.push(inner_vec);
        time_valids_vec.push(true);
    }
    set_val_in_time_space_vec(seq_idxs_to_vals, time_space_values_vec, time_valids_vec,
                              &st_type, total_width, total_time, valid_time, 0, 0, true, 0);
}

fn set_val_in_time_space_vec(seq_idx_to_vals: &mut HashMap<usize, Rc<String>>,
                             time_space_values_vec: &mut Vec<Vec<Rc<String>>>,
                             time_valids_vec: &mut Vec<bool>,
                             st_type: &Type, total_width: u32, total_time: u32,
                             valid_time: u32, cur_space: u32, cur_time: u32,
                             valid: bool, cur_idx: u32) {
    match st_type {
        Type::STuple { n, elem_type } => {
            let element_width = total_width / *n;
            let element_time = total_time;
            let element_valid_time = valid_time;
            for i in 0..=n - 1 {
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, time_valids_vec,
                                          elem_type, element_width, element_time,
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
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, time_valids_vec,
                                          elem_type, element_width, element_time,
                                          element_valid_time, cur_space + i * element_width,
                                          cur_time, valid,
                                          cur_idx + i * element_width * element_valid_time)
            }
        }
        Type::TSeq {n, i, elem_type} => {
            let element_width = total_width;
            let element_time = total_time / (*n + *i);
            let element_valid_time = valid_time / *n;
            for i in 0..=(n + i) - 1 {
                set_val_in_time_space_vec(seq_idx_to_vals, time_space_values_vec, time_valids_vec,
                                          elem_type, element_width, element_time,
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
            else if cur_space == 0 {
                time_valids_vec[cur_time as usize] = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_seq_val_to_st_val_string_sseq_4_int() {
        let mut vals_builder = Vec::new();
        let mut valids_builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4),
                                         Type::SSeq {n: 4, elem_type: Box::from(Type::Int)},
                                         &mut vals_builder, &mut valids_builder).unwrap();
        let vals_data = String::from_utf8(vals_builder).unwrap();
        assert_eq!(vals_data, String::from("[[1,3,2,4]]"));
        let valids_data = String::from_utf8(valids_builder).unwrap();
        assert_eq!(valids_data, String::from("[true]"));
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_4_int() {
        let mut vals_builder = Vec::new();
        let mut valids_builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4),
                                         Type::TSeq {n: 4, i: 0, elem_type: Box::from(Type::Int)},
                                         &mut vals_builder, &mut valids_builder).unwrap();
        let vals_data = String::from_utf8(vals_builder).unwrap();
        assert_eq!(vals_data, String::from("[1,3,2,4]"));
        let valids_data = String::from_utf8(valids_builder).unwrap();
        assert_eq!(valids_data, String::from("[true,true,true,true]"));
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_2_1_int() {
        let mut vals_builder = Vec::new();
        let mut valids_builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3),
                                         Type::TSeq {n: 2, i: 1, elem_type: Box::from(Type::Int)},
                                         &mut vals_builder, &mut valids_builder).unwrap();
        let vals_data = String::from_utf8(vals_builder).unwrap();
        assert_eq!(vals_data, String::from("[1,3,0]"));
        let valids_data = String::from_utf8(valids_builder).unwrap();
        assert_eq!(valids_data, String::from("[true,true,false]"));
    }

    #[test]
    fn test_convert_seq_val_to_st_val_string_tseq_3_0_sseq_2_int() {
        let mut vals_builder = Vec::new();
        let mut valids_builder = Vec::new();
        convert_seq_val_to_st_val_string(vec!(1,3,2,4,6,5),
                                         Type::TSeq {n: 3, i: 0, elem_type: Box::from(
                                             Type::SSeq {n: 2, elem_type: Box::from(Type::Int)})},
                                         &mut vals_builder, &mut valids_builder).unwrap();
        let vals_data = String::from_utf8(vals_builder).unwrap();
        assert_eq!(vals_data, String::from("[[1,3],[2,4],[6,5]]"));
        let valids_data = String::from_utf8(valids_builder).unwrap();
        assert_eq!(valids_data, String::from("[true,true,true]"));
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
