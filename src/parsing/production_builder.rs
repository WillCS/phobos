use std::collections::HashSet;

use crate::parsing::symbol::{Symbol, SymbolSequence};
use crate::parsing::production::Production;

pub struct ProductionBuilder<'t, T, U> {
    produced_symbol:  Option<U>,
    consumed_symbols: Option<SymbolSequence<T, U>>,
    lookahead_set:    Option<HashSet<Symbol<T, U>>>,
    reduce_handler:   Option<&'t dyn FnMut(Vec<Symbol<T, U>>) -> U>
}

impl<'t, T, U> ProductionBuilder<'t, T, U> {
    pub fn new() -> ProductionBuilder<'t, T, U> {
        ProductionBuilder {
            produced_symbol:  None,
            consumed_symbols: None,
            lookahead_set:    None,
            reduce_handler:   None
        }
    }
    pub fn producing(mut self,
        produced_symbol: U
    ) -> ProductionBuilder<'t, T, U> {
        self.produced_symbol = Some(produced_symbol);
        return self;
    }

    pub fn from(mut self,
        symbol_seq: SymbolSequence<T, U>
    ) -> ProductionBuilder<'t, T, U> {
        self.consumed_symbols = Some(symbol_seq);
        return self;
    }

    pub fn with_lookahead_set(mut self,
        lookahead_set: HashSet<Symbol<T, U>>
    ) -> ProductionBuilder<'t, T, U> {
        self.lookahead_set = Some(lookahead_set);
        return self;
    }

    pub fn build(self) -> Option<Production<'t, T, U>> {
        if 
            self.produced_symbol.is_some() &&
            self.consumed_symbols.is_some() &&
            self.lookahead_set.is_some() &&
            self.reduce_handler.is_some() {
            Some(Production::new(
                self.produced_symbol.unwrap(),
                self.consumed_symbols.unwrap(),
                self.lookahead_set.unwrap(),
                self.reduce_handler.unwrap()
            ))
        } else {
            None
        }
    }
}
