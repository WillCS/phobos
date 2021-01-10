use std::hash::Hash;
use std::fmt::Display;

pub trait NonterminalSymbol: Display + Clone + Copy + Eq + Hash {
    type NodeType;

    fn get_name(&self) -> &'static str;

    fn same_symbol(&self, other: &Self) -> bool;
}
