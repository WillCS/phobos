use std::fmt::{Display, Formatter};

use crate::parsing::nonterminal_symbol::NonterminalSymbol;
use crate::parsing::terminal_symbol::TerminalSymbol;

/** Symbol used internally within the parser. */
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Symbol<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    Terminal(T),
    Nonterminal(U),
    Empty
}

impl<T, U> Display for Symbol<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Symbol::Terminal(s)    => write!(f, "{}", s.get_name()),
            Symbol::Nonterminal(s) => write!(f, "{}", s.get_name()),
            Symbol::Empty          => write!(f, "ε")
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum PossiblyEmptyTerminalSymbol<T> where T: TerminalSymbol {
    Terminal(T),
    Empty
}

impl<T> Display for PossiblyEmptyTerminalSymbol<T> where T: TerminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            PossiblyEmptyTerminalSymbol::Terminal(s)    => write!(f, "{}", s.get_name()),
            PossiblyEmptyTerminalSymbol::Empty          => write!(f, "ε")
        }
    }
}
