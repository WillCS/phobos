use std::fmt::Formatter;
use std::fmt::Display;

use crate::parsing::{Symbol, SymbolSequence, ProductionBuilder, TerminalSymbol, NonterminalSymbol};

pub struct Production<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub produced_symbol:  U,
    pub consumed_symbols: SymbolSequence<T, U>,
    pub reduce_handler:   &'t dyn FnMut(Vec<Symbol<T, U>>) -> U
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

    pub fn produces(&self, symbol: U) -> bool {
        self.produced_symbol == symbol
    }
}

impl<'t, T, U> Display for Production<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} ::= {}", self.produced_symbol, self.consumed_symbols)
    }
}
