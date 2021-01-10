use std::hash::Hash;
use std::fmt::Display;

use crate::tokenisation::TokenData;

pub trait TerminalSymbol: Display + Clone + Copy + Eq + Hash {
    type DataType: TokenData;
    
    fn get_name(&self) -> &'static str;
}
