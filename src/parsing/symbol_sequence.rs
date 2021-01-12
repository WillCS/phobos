use crate::parsing::{Symbol, TerminalSymbol, NonterminalSymbol};

use std::fmt::{Display, Formatter, Write};
use std::vec::Vec;

/** Symbol used for construction of productions for the parser. */
#[derive(Clone)]
pub enum SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    Single(Symbol<T, U>),
    Sequence(Vec<SymbolSequence<T, U>>),
    Optional(Box<SymbolSequence<T, U>>),
    Repeated(Box<SymbolSequence<T, U>>),
    Alternatives(Vec<SymbolSequence<T, U>>),
}

impl<T, U> SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub fn from_terminal(terminal: T) -> SymbolSequence<T, U> {
        SymbolSequence::Single(Symbol::Terminal(terminal))
    }

    pub fn from_nonterminal(nonterminal: U) -> SymbolSequence<T, U> {
        SymbolSequence::Single(Symbol::Nonterminal(nonterminal))
    }

    pub fn either(
        opt1: SymbolSequence<T, U>,
        opt2: SymbolSequence<T, U>
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Alternatives(vec![opt1, opt2])
    }

    pub fn many(
        seq: SymbolSequence<T, U>,
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Repeated(Box::new(seq))
    }

    pub fn maybe(
        seq: SymbolSequence<T, U>,
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Optional(Box::new(seq))
    }
}

impl<T, U> From<Symbol<T, U>> for SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn from(symbol: Symbol<T, U>) -> SymbolSequence<T, U> {
        SymbolSequence::Single(symbol)
    }
}

impl<T, U> Display for SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SymbolSequence::Single(s)   => write!(f, "{}", s),
            SymbolSequence::Sequence(v) => {
                let mut output = "".to_string();
                let mut i = 1;
                for symbol in v {
                    write!(output, "{}", symbol).unwrap();
                    if i != v.len() {
                        write!(output, " ").unwrap();
                    }

                    i += 1;
                }

                write!(f, "{}", output)
            },
            SymbolSequence::Optional(s) => write!(f, "[ {} ]", s),
            SymbolSequence::Repeated(s) => write!(f, "{{ {} }}", s),
            SymbolSequence::Alternatives(v) => {
                let mut output = "".to_string();
                let mut i = 1;
                for symbol in v {
                    write!(output, "{}", symbol).unwrap();
                    if i != v.len() {
                        write!(output, " | ").unwrap();
                    }

                    i += 1;
                }

                write!(f, "{}", output)
            }
        }
    }
}
