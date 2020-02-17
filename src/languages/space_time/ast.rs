//! The deep embedding of Aetherling's Space-Time IR in Rust as an expression tree
use super::super::util::*;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Id,
    Abs,
    Add,
    MapS{n: SeqLen, f: Box<Node>},
    MapT{n: SeqLen, i: SeqLen, f: Box<Node>},
    Map2S{n: SeqLen, f: Box<Node>},
    Map2T{n: SeqLen, i: SeqLen, f: Box<Node>}
}

#[derive(Debug, PartialEq, Eq)]
pub enum NodeInputs {
    Nullary,
    Unary(Box<Node>),
    Binary{left: Box<Node>, right: Box<Node>}
}

/// A node object in an expression tree. An expression tree is formed by composing node
/// objects. THe root of the tree is the output node. The leafs are the inputs.
/// The each node's producers are indicated using the `inputs` field.
/// The compiler guarantees that `index` is unique per tree.
#[derive(Debug, PartialEq, Eq)]
pub struct Node{
    index: SeqLen,
    node_kind: NodeKind,
    inputs: NodeInputs
}

