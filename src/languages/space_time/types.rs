use super::super::util::*;

#[derive(Debug)]
pub enum Type {
    Unit,
    Bit,
    Int,
    ATuple{left: Box<Type>, right: Box<Type>},
    STuple{n: SeqLen, elem_type: Box<Type>},
    SSeq{n: SeqLen, elem_type: Box<Type>},
    TSeq{n: SeqLen, i: SeqLen, elem_type: Box<Type>}
}