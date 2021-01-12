use std::collections::HashSet;

use crate::parsing::{ParserBuilder, TerminalSymbol, NonterminalSymbol, SymbolSequence, PossiblyEmptyTerminalSymbol, Symbol};

impl<'t, T, U> ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub(super) fn derive_first_sets(&mut self) {
        U::into_enum_iter().map(|nonterminal| {
            self.get_first_set_for(&nonterminal);
        });
    }
    
    fn derive_first_set_for(&mut self, non_terminal: U) -> &HashSet<PossiblyEmptyTerminalSymbol<T>> {
        let mut first_set = HashSet::new();

        for p in &self.productions {
            if p.produces(non_terminal) {
                let symbol_seq = &p.consumed_symbols;
                self.derive_first_set_for_symbol_sequence(symbol_seq, &mut first_set);
            }
        }

        if !self.first_sets.contains_key(&non_terminal) {
            self.first_sets.insert(non_terminal, first_set);
        }

        return self.first_sets.get(&non_terminal).unwrap();
    }

    fn derive_first_set_for_symbol_sequence(&mut self, 
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        match seq {
            SymbolSequence::Single(s)       => self.derive_first_set_for_symbol(s, first_set),
            SymbolSequence::Sequence(v)     => self.derive_first_set_for_sequence(v, first_set),
            SymbolSequence::Optional(s)     => self.derive_first_set_for_optional(&s, first_set),
            SymbolSequence::Repeated(s)     => self.derive_first_set_for_repeated(&s, first_set),
            SymbolSequence::Alternatives(v) => self.derive_first_set_for_alternatives(&v, first_set),
        };
    }

    fn derive_first_set_for_sequence(&mut self,
        seq:       &Vec<SymbolSequence<T, U>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        let first = seq.first();

        match first {
            Some(s) => self.derive_first_set_for_symbol_sequence(s, first_set),
            None    => panic!()
        }
    }

    fn derive_first_set_for_optional(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set);
    }

    fn derive_first_set_for_repeated(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set);
    }

    fn derive_first_set_for_alternatives(&mut self,
        alts:      &Vec<SymbolSequence<T, U>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        for alt in alts {
            self.derive_first_set_for_symbol_sequence(alt, first_set);
        }
    }

    fn first_set_repeated_optional_thunk(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self.derive_first_set_for_symbol_sequence(seq, first_set);
        first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    }

    fn derive_first_set_for_symbol(&mut self,
        symbol:    &Symbol<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        match symbol {
            Symbol::Terminal(s)    => { first_set.insert(PossiblyEmptyTerminalSymbol::Terminal(*s)); },
            Symbol::Nonterminal(n) => { first_set.extend(self.get_first_set_for(n).clone()); },
            Symbol::Empty          => { first_set.insert(PossiblyEmptyTerminalSymbol::Empty); }
        }
    }

    fn get_first_set_for(&mut self, non_terminal: &U) -> &HashSet<PossiblyEmptyTerminalSymbol<T>> {
        if self.first_sets.contains_key(&non_terminal) {
            return self.first_sets.get(&non_terminal).unwrap();
        } else {
            return self.derive_first_set_for(*non_terminal);
        }
    }
}
