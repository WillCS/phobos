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
