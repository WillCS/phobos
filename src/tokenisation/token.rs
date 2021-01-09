use std::fmt::Display;
use std::fmt::Formatter;

use crate::parsing::TerminalSymbol;

#[derive(Debug)]
pub struct Token<T> where T: TerminalSymbol {
    pub token_type: T,
    pub location:   Location
}

impl<T: Display> Display for Token<T> where T: TerminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} at {}", self.token_type, self.location)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub col:  usize
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}
