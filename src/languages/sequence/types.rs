//! The deep embedding of Aetherling's Sequence types in Rust
use super::super::util::*;

/// A type of a Sequence Languages expression's input or output.
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
    Seq{n: SeqLen, elem_type: Box<Type>},
}

