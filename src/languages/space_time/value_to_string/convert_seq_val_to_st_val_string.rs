use super::to_atom_strings::ToAtomStrings;
use super::super::types::Type;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use std::io::Write;
/*
convert_seq_val_to_st_val_string ::
  Convertible_To_Atom_Strings a => a -> AST_Type ->
  ST_Val_To_String_Config -> ST_Val_String
convert_seq_val_to_st_val_string seq_val st_type conf = do
  -- get the mapping from flat_idx to value as a string
  let flat_val_strs = convert_to_flat_atom_list seq_val conf
  let flat_val_idx_to_str :: M.Map Int String =
        M.fromList $ zip [0..] flat_val_strs

  -- get the mapping from flat st to flat_idx
  let st_vals = generate_st_val_idxs_for_st_type_new flat_val_idx_to_str st_type
  --let valid_clks = map mv_valid $ map head st_idxs

  -- convert the st_idx double nested arrays to st double arrays with values
  --let st_vals = convert_st_val_idxs_to_vals flat_val_idx_to_str st_idxs
  -- these are nested for both space and time
  -- issue: if 1 input per clock, then need to remove the space dimension
  -- as each input port is not vectorized
  let st_val_string = make_array_string_for_backend conf $
                      remove_sseq_length_one conf st_vals
  ST_Val_String st_val_string [True]
*/

pub fn convert_seq_val_to_st_val_string<T: ToAtomStrings, W: Write>(seq_val: T, st_type: Type, sink: &mut W) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(sink);
    let mut flat_val_strs: Vec<Rc<String>> = Vec::new();
    seq_val.convert_to_flat_atom_list(&mut flat_val_strs, true);
    let mut flat_val_idx_to_str: HashMap<usize, Rc<String>> = flat_val_strs.into_iter().enumerate().collect();

    let st_vals = convert_seq_idxs_to_vals_to_time_space_vec(&mut flat_val_idx_to_str, st_type);
    wtr.serialize(st_vals)?;
    wtr.flush()?;
    Ok(())
}

/*
generate_st_val_idxs_for_st_type_new :: M.Map Int String -> AST_Type -> [[String]]
generate_st_val_idxs_for_st_type_new idx_to_str t = do
  let total_width = num_atoms_per_valid_t t
  let total_time = clocks_t t
  let valid_time = valid_clocks_t t
  --let initial_idxs = newArray ((0,0),(total_time-1,total_width-1)) (ST_Val_Index 0 False 0 0)
  --set_val_index t total_width total_time valid_time 0 0 True 0 initial_idxs
  let arr = runSTArray $ initialize_and_set_val_indexes idx_to_str t total_width total_time
            valid_time
  [[ arr Arr.! (t, s) | s <- [0..total_width - 1]] | t <- [0..total_time-1]]

*/

fn convert_seq_idxs_to_vals_to_time_space_vec(seq_idxs_to_vals: &mut HashMap<usize, Rc<String>>,
                                              st_type: Type) -> Vec<Vec<Rc<String>>> {
    let total_width = st_type.atoms_per_valid();
    let total_time = st_type.clocks();
    let valid_time = st_type.valid_clocks();
    let mut time_space_values_vec: Vec<Vec<Rc<String>>> = Vec::with_capacity(total_time as usize);
    for v in time_space_values_vec.iter_mut() {
        *v = Vec::with_capacity(total_width as usize);
    };
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
        assert_eq!(data, String::from("[1,3,2,4]"))
    }
}
/*
set_val_index idx_to_str (STupleT n t) total_width
  total_time valid_time cur_space cur_time valid cur_idx st_val_idxs = do
  let element_width = total_width `div` n
  let element_time = total_time
  let element_valid_time = valid_time
  foldM'
    (\_ j -> set_val_index idx_to_str t
           element_width element_time element_valid_time
           (cur_space + j*element_width) cur_time
           valid
           (cur_idx + j*element_width*element_valid_time)
           st_val_idxs
    ) () [0..n-1]
  return ()
set_val_index idx_to_str (SSeqT n t) total_width
  total_time valid_time cur_space cur_time valid cur_idx st_val_idxs = do
  let element_width = total_width `div` n
  let element_time = total_time
  let element_valid_time = valid_time
  foldM'
    (\_ j -> set_val_index idx_to_str t
           element_width element_time element_valid_time
           (cur_space + j*element_width) cur_time
           valid
           (cur_idx + j*element_width*element_valid_time)
           st_val_idxs
    ) () [0..n-1]
  return ()
set_val_index idx_to_str (TSeqT n i t) total_width
  total_time valid_time cur_space cur_time valid cur_idx st_val_idxs = do
  let element_width = total_width
  let element_time = total_time `div` (n+i)
  let element_valid_time = valid_time `div` n
  foldM'
    (\_ j -> set_val_index idx_to_str t
           element_width element_time element_valid_time
           cur_space (cur_time + j * element_time)
           (valid && j < n)
           (cur_idx + j*element_width*element_valid_time)
           st_val_idxs
    ) () [0..(n+i)-1]
  return ()
set_val_index idx_to_str _ _ _ _ cur_space cur_time valid cur_idx st_val_idxs = do
  writeArray st_val_idxs (cur_time, cur_space)
    (M.findWithDefault "0" cur_idx idx_to_str)

*/
