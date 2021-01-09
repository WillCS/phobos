use std::fmt::Formatter;
use std::fmt::Display;

use crate::parsing::symbol::{Symbol, SymbolSequence};
use crate::parsing::production_builder::ProductionBuilder;
use crate::parsing::terminal_symbol::TerminalSymbol;
use crate::parsing::nonterminal_symbol::NonterminalSymbol;

pub struct Production<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    produced_symbol:  U,
    consumed_symbols: SymbolSequence<T, U>,
    reduce_handler:   &'t dyn FnMut(Vec<Symbol<T, U>>) -> U
}

impl<'t, T, U> Production<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub fn builder() -> ProductionBuilder<'t, T, U> {
        ProductionBuilder::new()
    }

    pub fn new(
        produced_symbol:  U,
        consumed_symbols: SymbolSequence<T, U>,
        reduce_handler:   &'t dyn FnMut(Vec<Symbol<T, U>>) -> U
    ) -> Production<'t, T, U> {
        Production {
            produced_symbol,
            consumed_symbols,
            reduce_handler
        }
    }
}

impl<'t, T, U> Display for Production<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ::= {}", self.produced_symbol, self.consumed_symbols)
    }
}
