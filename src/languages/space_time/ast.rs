//! The deep embedding of Aetherling's Space-Time IR in Rust as an expression tree
use super::super::util::*;

/// The type of node object
#[derive(Debug)]
pub enum NodeKind {
    Id,
    Abs,
    Add,
    MapS{n: SeqLen, f: Box<Node>},
    MapT{n: SeqLen, i: SeqLen, f: Box<Node>},
    Map2S{n: SeqLen, f: Box<Node>},
    Map2T{n: SeqLen, i: SeqLen, f: Box<Node>}
}

pub enum NodeInputs {
    Nullary,
    Unary(Box<Node>),
    Binary{left: Box<Node>, right: Box<Node>}
}

/// A node object in an expression tree
pub struct Node{
    index: SeqLen,
    node_kind: NodeKind,
    inputs: NodeInputs
}

