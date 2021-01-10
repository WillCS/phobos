use std::fmt::Display;

pub trait NonterminalSymbol: Display {
    type NodeType;

    fn get_name(&self) -> &'static str;
}
