use std::collections::HashSet;
use std::iter::FromIterator;

use crate::parsing::{ParserBuilder, TerminalSymbol, NonterminalSymbol, SymbolSequence, Symbol};

impl<'t, T, U> ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub(super) fn derive_first_sets(&mut self) {
        for p in &self.productions {
            self.get_first_set_for(p.produced_symbol);
        }
    }

    fn derive_first_set_for(&mut self, non_terminal: U) -> &HashSet<T> {
        let mut first_set = HashSet::new();

        for p in &self.productions {
            if p.produces(non_terminal) {
                first_set.extend(self.derive_first_set_for_symbol_sequence(&p.consumed_symbols));
            }
        }

        if !self.first_sets.contains_key(&non_terminal) {
            self.first_sets.insert(non_terminal, first_set);
        }

        return self.first_sets.get(&non_terminal).unwrap();
    }

    fn derive_first_set_for_symbol_sequence(&mut self, seq: &SymbolSequence<T, U>) -> HashSet<T> {
        match seq {
            SymbolSequence::Single(s)       => self.derive_first_set_for_symbol(s),
            SymbolSequence::Sequence(v)     => self.derive_first_set_for_sequence(&v),
            SymbolSequence::Optional(s)     => self.derive_first_set_for_optional(&s),
            SymbolSequence::Repeated(s)     => self.derive_first_set_for_repeated(&s),
            SymbolSequence::Alternatives(v) => self.derive_first_set_for_alternatives(&v),
        }
    }

    fn derive_first_set_for_sequence(&mut self, seq: &Vec<SymbolSequence<T, U>>) -> HashSet<T> {
        let first = seq.first();

        match (first, self.empty_symbol) {
            (Some(s), _)    => self.derive_first_set_for_symbol_sequence(s),
            (None, Some(e)) => HashSet::from_iter(vec![e]),
            (None, None)    => panic!()
        }
    }

    fn derive_first_set_for_optional(&mut self, seq: &SymbolSequence<T, U>) -> HashSet<T> {
        self.first_set_repeated_optional_thunk(seq)
    }

    fn derive_first_set_for_repeated(&mut self, seq: &SymbolSequence<T, U>) -> HashSet<T> {
        self.first_set_repeated_optional_thunk(seq)
    }

    fn derive_first_set_for_alternatives(&mut self, alts: &Vec<SymbolSequence<T, U>>) -> HashSet<T> {
        let mut first_set = HashSet::new();

        for alt in alts {
            first_set.extend(self.derive_first_set_for_symbol_sequence(alt))
        }

        return first_set;
    }

    fn first_set_repeated_optional_thunk(&mut self, seq: &SymbolSequence<T, U>) -> HashSet<T> {
        let mut first_set = self.derive_first_set_for_symbol_sequence(seq);

        match self.empty_symbol {
            Some(e) => first_set.insert(e),
            None    => panic!()
        };

        return first_set;
    }

    fn derive_first_set_for_symbol(&mut self, symbol: &Symbol<T, U>) -> HashSet<T> {
        match symbol {
            Symbol::Terminal(s)    => HashSet::from_iter(vec![*s]),
            Symbol::Nonterminal(n) => {
                let mut set = HashSet::new();
                set.extend(self.get_first_set_for(*n));
                return set;
            }
        }
    }

    fn get_first_set_for(&mut self, non_terminal: U) -> &HashSet<T> {
        if self.first_sets.contains_key(&non_terminal) {
            return self.first_sets.get(&non_terminal).unwrap();
        } else {
            return self.derive_first_set_for(non_terminal);
        }
    }
}
