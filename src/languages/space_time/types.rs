//! The deep embedding of Aetherling's Space-Time types in Rust
use super::super::util::*;

/// A type of a Space-Time IR expression's input or output.
#[derive(Debug, PartialEq)]
pub enum Type {
    Unit,
    Bit,
    Int8,
    UInt8,
    Int16,
    UInt16,
    Int32,
    UInt32,
    ATuple{left: Box<Type>, right: Box<Type>},
    STuple{n: SeqLen, elem_type: Box<Type>},
    SSeq{n: SeqLen, elem_type: Box<Type>},
    TSeq{n: SeqLen, i: SeqLen, elem_type: Box<Type>}
}

impl Type {
    /// Compute the size in bits of a type.
    ///
    /// # Examples
    ///
    /// ```
    /// use aetherling::languages::space_time::types::Type;
    /// let bit_size = Type::Bit.size();
    ///
    /// assert_eq!(bit_size, 1);
    /// ```
    pub fn size(&self) -> u32 {
        match self {
            Type::Unit => 0,
            Type::Bit=> 1,
            Type::Int8 => 8,
            Type::UInt8 => 8,
            Type::Int16 => 16,
            Type::UInt16 => 16,
            Type::Int32 => 32,
            Type::UInt32 => 32,
            Type::ATuple{ left, right } => left.size() + right.size(),
            Type::STuple { n, elem_type } => *n * elem_type.size(),
            Type::SSeq { n, elem_type } => *n * elem_type.size(),
            Type::TSeq { n: _, i: _, elem_type } => elem_type.size()
        }
    }

    /// Compute the number of atoms per valid clock
    ///
    /// # Examples
    ///
    /// ```
    /// use aetherling::languages::space_time::types::Type;
    /// let t = Type::SSeq {n: 3, elem_type: Box::from(Type::Bit)};
    ///
    /// assert_eq!(t.atoms_per_valid(), 3);
    /// ```
    pub fn atoms_per_valid(&self) -> u32 {
        match self {
            Type::Unit => 1,
            Type::Bit=> 1,
            Type::Int8 => 1,
            Type::UInt8 => 1,
            Type::Int16 => 1,
            Type::UInt16 => 1,
            Type::Int32 => 1,
            Type::UInt32 => 1,
            Type::ATuple{ .. } => 1,
            Type::STuple { n, elem_type } => *n * elem_type.atoms_per_valid(),
            Type::SSeq { n, elem_type } => *n * elem_type.atoms_per_valid(),
            Type::TSeq { n: _, i: _, elem_type } => elem_type.atoms_per_valid()
        }
    }

    /// Compute the number of clock cycles for a type.
    /// Note that technically, atom's don't take a number of clocks.
    /// They are all 1 to simplify this operation's implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use aetherling::languages::space_time::types::Type;
    /// let t = Type::TSeq {n:4, i:2, elem_type:Box::from(Type::UInt8)};
    ///
    /// assert_eq!(t.clocks(), 6)
    /// ```
    pub fn clocks(&self) -> u32 {
        match self {
            Type::Unit => 1,
            Type::Bit=> 1,
            Type::Int8 => 1,
            Type::UInt8 => 1,
            Type::Int16 => 1,
            Type::UInt16 => 1,
            Type::Int32 => 1,
            Type::UInt32 => 1,
            Type::ATuple{ .. } => 1,
            Type::STuple { n: _, elem_type } => elem_type.clocks(),
            Type::SSeq { n: _, elem_type } => elem_type.clocks(),
            Type::TSeq { n, i, elem_type } => (*n+*i) * elem_type.clocks()
        }
    }

    /// Compute the number of valid clock cycles for a type.
    /// Note that technically, atom's don't take a number of clocks.
    /// They are all 1 to simplify this operation's implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use aetherling::languages::space_time::types::Type;
    /// let t = Type::TSeq {n:4, i:2, elem_type:Box::from(Type::UInt8)};
    ///
    /// assert_eq!(t.valid_clocks(), 4)
    /// ```
    pub fn valid_clocks(&self) -> u32 {
        match self {
            Type::Unit => 1,
            Type::Bit=> 1,
            Type::Int8 => 1,
            Type::UInt8 => 1,
            Type::Int16 => 1,
            Type::UInt16 => 1,
            Type::Int32 => 1,
            Type::UInt32 => 1,
            Type::ATuple{ .. } => 1,
            Type::STuple { n: _, elem_type } => elem_type.valid_clocks(),
            Type::SSeq { n: _, elem_type } => elem_type.valid_clocks(),
            Type::TSeq { n, i: _, elem_type } => *n * elem_type.valid_clocks()
        }
    }

    pub fn def_atom_str(&self) -> String {
        match self {
            Type::Unit => "()".to_string(),
            Type::Bit => "False".to_string(),
            Type::Int8 => "0".to_string(),
            Type::UInt8 => "0".to_string(),
            Type::Int16 => "0".to_string(),
            Type::UInt16 => "0".to_string(),
            Type::Int32 => "0".to_string(),
            Type::UInt32 => "0".to_string(),
            Type::ATuple {left, right } => format!("[{},{}]", left.def_atom_str(), right.def_atom_str()),
            Type::STuple {n: _, elem_type} => elem_type.def_atom_str(),
            Type::SSeq {n: _, elem_type} => elem_type.def_atom_str(),
            Type::TSeq {n: _, i: _, elem_type} => elem_type.def_atom_str()
        }
    }
}

/*

normalize_type = merge_layers . strip_empty_layers . replace_stuple_with_sseq

strip_empty_layers :: AST_Type -> AST_Type
strip_empty_layers (SSeqT 1 t) = strip_empty_layers t
strip_empty_layers (SSeqT n t) = SSeqT n $ strip_empty_layers t
strip_empty_layers (TSeqT 1 0 t) = strip_empty_layers t
strip_empty_layers (TSeqT n i t) = TSeqT n i $ strip_empty_layers t
strip_empty_layers (STupleT 1 t) = strip_empty_layers t
strip_empty_layers (STupleT n t) = STupleT n $ strip_empty_layers t
strip_empty_layers t = t

-- merge types where it doesn't change ordering of valids/invalids
merge_layers :: AST_Type -> AST_Type
merge_layers (SSeqT no (SSeqT ni t)) = merge_layers $ SSeqT (no*ni) t
merge_layers (SSeqT no (STupleT ni t)) = merge_layers $ SSeqT (no*ni) t
merge_layers (TSeqT no io (TSeqT ni 0 t)) = merge_layers $ TSeqT (no*ni) (io*ni) t
merge_layers (STupleT no (STupleT ni t)) = merge_layers $ SSeqT (no*ni) t
merge_layers (STupleT no (SSeqT ni t)) = merge_layers $ SSeqT (no*ni) t
merge_layers (SSeqT no t) = SSeqT no $ merge_layers t
merge_layers (TSeqT no io t) = TSeqT no io $ merge_layers t
merge_layers (STupleT no t) = SSeqT no $ merge_layers t
merge_layers t = t

replace_stuple_with_sseq :: AST_Type -> AST_Type
replace_stuple_with_sseq (STupleT n t) = SSeqT n $ replace_stuple_with_sseq t
replace_stuple_with_sseq (SSeqT n t) = SSeqT n $ replace_stuple_with_sseq t
replace_stuple_with_sseq (TSeqT n i t) = TSeqT n i $ replace_stuple_with_sseq t
replace_stuple_with_sseq t = t

diff_types :: AST_Type -> AST_Type -> Maybe AST_Type
diff_types a b | a == b = Nothing
diff_types (SSeqT na a) (SSeqT nb b) | na == nb = diff_types a b
diff_types (TSeqT na ia a) (TSeqT nb ib b) | (na == nb) && (ia == ib) = diff_types a b
diff_types (STupleT na a) (STupleT nb b) | na == nb = diff_types a b
diff_types a b = Just a

*/