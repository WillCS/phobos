use std::collections::HashSet;

use crate::parsing::{ParserBuilder, TerminalSymbol, NonterminalSymbol, SymbolSequence, PossiblyEmptyTerminalSymbol, Symbol};

impl<'t, T, U> ParserBuilder<'t, T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub(super) fn derive_first_sets(&mut self) {
        U::into_enum_iter().map(|nonterminal| {
            let mut break_set = Vec::new();
            self.get_first_set_for(&nonterminal, &mut break_set);
        }).for_each(drop);
    }
    
    fn derive_first_set_for(&mut self, non_terminal: U, break_set: &mut Vec<U>) -> &HashSet<PossiblyEmptyTerminalSymbol<T>> {
        let mut first_set = HashSet::new();

        let mut relevant_productions = Vec::new();

        for p in &self.productions {
            if p.produces(non_terminal) {
                relevant_productions.push(p.consumed_symbols.clone());
            }
        }

        for p in relevant_productions {
            self.derive_first_set_for_symbol_sequence(&p, &mut first_set, break_set);
        }

        self.first_sets.insert(non_terminal, first_set);

        return self.first_sets.get(&non_terminal).unwrap();
    }

    fn derive_first_set_for_symbol_sequence(&mut self, 
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        match seq {
            SymbolSequence::Single(s)       => self.derive_first_set_for_symbol(s, first_set, break_set),
            SymbolSequence::Sequence(v)     => self.derive_first_set_for_sequence(v, first_set, break_set),
            SymbolSequence::Optional(s)     => self.derive_first_set_for_optional(&s, first_set, break_set),
            SymbolSequence::Repeated(s)     => self.derive_first_set_for_repeated(&s, first_set, break_set),
            SymbolSequence::Alternatives(v) => self.derive_first_set_for_alternatives(&v, first_set, break_set),
        };
    }

    fn derive_first_set_for_sequence(&mut self,
        seq:       &Vec<SymbolSequence<T, U>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        for s in seq {
            let mut first_set_of_s = HashSet::new();
            self.derive_first_set_for_symbol_sequence(&s, &mut first_set_of_s, break_set);

            let reduces_to_empty = first_set_of_s.contains(&PossiblyEmptyTerminalSymbol::Empty);
            first_set.extend(first_set_of_s);

            if !reduces_to_empty {
                break;
            }
        }
    }

    fn derive_first_set_for_optional(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    }

    fn derive_first_set_for_repeated(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    }

    fn derive_first_set_for_alternatives(&mut self,
        alts:      &Vec<SymbolSequence<T, U>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        for alt in alts {
            self.derive_first_set_for_symbol_sequence(alt, first_set, break_set);
        }
    }

    fn first_set_repeated_optional_thunk(&mut self,
        seq:       &SymbolSequence<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        self.derive_first_set_for_symbol_sequence(seq, first_set, break_set);
        first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    }

    fn derive_first_set_for_symbol(&mut self,
        symbol:    &Symbol<T, U>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<U>
    ) {
        match symbol {
            Symbol::Terminal(s)    => { first_set.insert(PossiblyEmptyTerminalSymbol::Terminal(*s)); },
            Symbol::Nonterminal(n) => { 
                if !break_set.contains(n) {
                    break_set.push(*n);
                    first_set.extend(self.get_first_set_for(n, break_set).clone());
                }
            },
            Symbol::Empty          => { first_set.insert(PossiblyEmptyTerminalSymbol::Empty); }
        }
    }

    fn get_first_set_for(&mut self, non_terminal: &U, break_set: &mut Vec<U>) -> &HashSet<PossiblyEmptyTerminalSymbol<T>> {
        if self.first_sets.contains_key(&non_terminal) {
            return self.first_sets.get(&non_terminal).unwrap();
        } else {
            return self.derive_first_set_for(*non_terminal, break_set);
        }
    }
}
