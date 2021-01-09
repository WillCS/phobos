use crate::tokenisation::TokenData;
use std::fmt::Display;

pub trait TerminalSymbol: Display + Clone {
    type DataType: TokenData;
    
    fn get_name(&self) -> &'static str;
}