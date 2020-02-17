use super::to_atom_strings::ToAtomStrings;
use super::super::types::Type;
use std::collections::HashMap;
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

#[derive(Debug)]
pub struct STValsAndValids {
    values: String,
    valids: String
}

pub fn convert_seq_val_to_st_val_string<T: ToAtomStrings>(seq_val: T, t: Type) -> STValsAndValids {
    let mut flat_val_strs: Vec<String> = Vec::new();
    seq_val.convert_to_flat_atom_list(&mut flat_val_strs, true);
    let flat_val_idx_to_str: HashMap<usize, String> = flat_val_strs.into_iter().enumerate().collect();

    STValsAndValids{values: String::from(""), valids: String::from("")}
}
