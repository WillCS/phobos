use std::collections::{HashMap, HashSet};

use crate::parsing::{TerminalSymbol, NonterminalSymbol, Production};

mod first_set;

pub struct ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    productions:  Vec<Production<'t, T, U>>,
    first_sets:   HashMap<U, HashSet<T>>,
    follow_sets:  HashMap<U, HashSet<T>>,
    start_symbol: Option<U>,
    empty_symbol: Option<T>
}

impl<'t, T, U> ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub fn new() -> ParserBuilder<'t, T, U> {
        ParserBuilder {
            productions:  Vec::new(),
            first_sets:   HashMap::new(),
            follow_sets:  HashMap::new(),
            start_symbol: None,
            empty_symbol: None
        }
    }

    pub fn with_productions(mut self,
        productions: &mut Vec<Production<'t, T, U>>
    ) -> ParserBuilder<'t, T, U> {
        self.productions.append(productions);
        return self;
    }

    pub fn with_start_symbol(mut self,
        start_symbol: U
    ) -> ParserBuilder<'t, T, U> {
        self.start_symbol = Some(start_symbol);
        return self;
    }

    pub fn with_empty_symbol(mut self,
        empty_symbol: T
    ) -> ParserBuilder<'t, T, U> {
        self.empty_symbol = Some(empty_symbol);
        return self;
    }

    pub fn build(mut self) {
        self.derive_first_sets();

        for (k, v) in self.first_sets {
            println!("First set of {}", k);
            for t in v {
                println!("- {}", t);
            }
        }
    }
}
