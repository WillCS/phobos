use std::collections::{HashMap, HashSet};

use crate::parsing::{PossiblyEmptyTerminalSymbol, PossiblyEndOfFileTerminalSymbol, TerminalSymbol, NonterminalSymbol, Production};

mod first_set;
mod follow_set;

pub struct ParserBuilder<'t, T, N> where T: TerminalSymbol, N: NonterminalSymbol {
    productions:  Vec<Production<'t, T, N>>,
    first_sets:   HashMap<N, HashSet<PossiblyEmptyTerminalSymbol<T>>>,
    follow_sets:  HashMap<N, HashSet<PossiblyEndOfFileTerminalSymbol<T>>>,
    start_symbol: Option<N>,
}

impl<'t, T, U> ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub fn new() -> ParserBuilder<'t, T, U> {
        ParserBuilder {
            productions:  Vec::new(),
            first_sets:   HashMap::new(),
            follow_sets:  HashMap::new(),
            start_symbol: None,
        }
    }

    pub fn with_production(mut self,
        production: Production<'t, T, U>
    ) -> ParserBuilder<'t, T, U> {

        self.productions.push(production);
        return self;
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

    pub fn build(mut self) {
        self.derive_first_sets();
        self.derive_follow_sets();

        for (k, v) in self.first_sets {
            println!("First set of {}", k);
            for t in v {
                println!("- {}", t);
            }
        }
    }
    
    /** Helper method to determine whether a production producing the given
     *  nonterminal has been added or not. */
    fn production_exists_for(&self, nonterminal: U) -> bool {
        for p in &self.productions {
            if p.produced_symbol == nonterminal {
                return true;
            }
        }

        return false;
    }
}
