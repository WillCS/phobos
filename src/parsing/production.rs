use std::fmt::Formatter;
use std::fmt::Display;
use std::collections::HashSet;

use crate::parsing::symbol::{Symbol, SymbolSequence};
use crate::parsing::production_builder::ProductionBuilder;

pub struct Production<'t, T, U> {
    produced_symbol:  U,
    consumed_symbols: SymbolSequence<T, U>,
    lookahead_set:    HashSet<Symbol<T, U>>,
    reduce_handler:   &'t dyn FnMut(Vec<Symbol<T, U>>) -> U
}

impl<'t, T, U> Production<'t, T, U> {
    pub fn builder() -> ProductionBuilder<'t, T, U> {
        ProductionBuilder::new()
    }

    pub fn new(
        produced_symbol:  U,
        consumed_symbols: SymbolSequence<T, U>,
        lookahead_set:    HashSet<Symbol<T, U>>,
        reduce_handler:   &'t dyn FnMut(Vec<Symbol<T, U>>) -> U
    ) -> Production<'t, T, U> {
        Production {
            produced_symbol,
            consumed_symbols,
            lookahead_set,
            reduce_handler
        }
    }
}

impl<'t, T, U> Display for Production<'t, T, U> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.lookahead_set.is_empty() {
            write!(f, "{} ::= {}", self.produced_symbol, self.consumed_symbols)
        } else {
            write!(f, "{} ::= {}, {}", self.produced_symbol, self.consumed_symbols, self.lookahead_set)
        }
    }
}
