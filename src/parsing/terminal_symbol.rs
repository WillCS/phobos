use std::hash::Hash;
use std::fmt::Display;

use enum_iterator::IntoEnumIterator;

use crate::tokenisation::TokenData;

pub trait TerminalSymbol: Display + Clone + Copy + Eq + Hash + IntoEnumIterator {
    type DataType: TokenData;
    
    fn get_name(&self) -> &'static str;
}
