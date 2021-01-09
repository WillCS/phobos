use std::fmt::Display;

pub trait TerminalSymbol: Display + Clone {
    fn get_name(&self) -> &'static str;
}