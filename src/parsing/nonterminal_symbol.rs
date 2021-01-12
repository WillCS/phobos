use std::hash::Hash;
use std::fmt::Display;

use enum_iterator::IntoEnumIterator;

pub trait NonterminalSymbol: Display + Clone + Copy + Eq + Hash + IntoEnumIterator {
    type NodeType;

    fn get_name(&self) -> &'static str;

    fn same_symbol(&self, other: &Self) -> bool;
}
