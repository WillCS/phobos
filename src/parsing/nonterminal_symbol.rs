use std::fmt::Display;

pub trait NonterminalSymbol: Display {
    fn get_name(&self) -> &'static str;
}