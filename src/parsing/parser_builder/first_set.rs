use std::collections::HashSet;

use crate::parsing::{ParserBuilder, TerminalSymbol, NonterminalSymbol, SymbolSequence, PossiblyEmptyTerminalSymbol, Symbol};

/** 
 * This file contains functions relating to the derivation of First Sets. 
 * 
 * The First Set, FIRST(a) for symbol a is the set of terminal symbols that can appear
 * at the beginning of strings derived from a.
 * 
 * First Sets are used in the derivation of Follow Sets.
 */
impl<'t, T, N> ParserBuilder<'t, T, N> where T: TerminalSymbol, N: NonterminalSymbol {
    
    /**
     * Derives the First Sets for all nonterminal symbols and stores them in the
     * struct's list of First Sets.
     */
    pub(super) fn derive_first_sets(&mut self) {
        N::into_enum_iter()
            .for_each(|nonterminal| {
                let mut break_set = Vec::new();
                let first_set = self._derive_first_set(nonterminal, &mut break_set);

                self.first_sets.insert(nonterminal, first_set);
            });
    }

    /**
     * Calculates the First Set for any given sequence of symbols.
     */
    pub(super) fn first_set(&self, seq: SymbolSequence<T, N>) -> HashSet<PossiblyEmptyTerminalSymbol<T>> {
        let mut first_set = HashSet::new();
        self._first_set(&seq, &mut first_set);

        return first_set;
    }

    fn _first_set(&self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    ) {
        match seq {
            SymbolSequence::Single(sym)       => self.get_first_set_for_symbol(sym, first_set),
            SymbolSequence::Sequence(vec)     => self.get_first_set_for_sequence(vec, first_set),
            SymbolSequence::Optional(sym)     => self.get_first_set_for_optional(sym, first_set),
            SymbolSequence::Repeated(sym)     => self.get_first_set_for_repeated(sym, first_set),
            SymbolSequence::Alternatives(vec) => self.get_first_set_for_alternatives(vec, first_set)
        }
    }

    fn get_first_set_for_symbol(&self,
        sym:       &Symbol<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        match sym {
            Symbol::Terminal(t)    => { first_set.insert(PossiblyEmptyTerminalSymbol::Terminal(*t)); },
            Symbol::Nonterminal(n) => { first_set.extend(self.first_sets[&n]) },
            Symbol::Empty          => { first_set.insert(PossiblyEmptyTerminalSymbol::Empty); }
        }
    }

    fn get_first_set_for_sequence(&self, 
        seq:       &Vec<SymbolSequence<T, N>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        for s in seq {
            let mut first_set_of_s = HashSet::new();
            self._first_set(s, &mut first_set_of_s);

            let reduces_to_empty = first_set_of_s.contains(&PossiblyEmptyTerminalSymbol::Empty);
            first_set.extend(first_set_of_s);

            if !reduces_to_empty {
                break;
            }
        }
    }

    fn get_first_set_for_optional(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self.get_repeated_optional_thunk(seq, first_set);
    }

    fn get_first_set_for_repeated(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self.get_repeated_optional_thunk(seq, first_set);
    }

    fn get_first_set_for_alternatives(&mut self,
        alts:      &Vec<SymbolSequence<T, N>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        for alt in alts {
            self._first_set(alt, first_set);
        }
    }

    fn get_repeated_optional_thunk(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>
    ) {
        self._first_set(seq, first_set);
        first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    }

    /**
     * Derives and returns the first set for the specified nonterminal symbol.
     */
    fn _derive_first_set(&mut self,
        non_terminal: N, 
        break_set:    &mut Vec<N>
    ) -> HashSet<PossiblyEmptyTerminalSymbol<T>> {
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

        return first_set;
    }

    /**
     * Delegates how the First Set of a SymbolSequence should be derived based 
     * on what kind of SymbolSequence it is.
     */
    fn derive_first_set_for_symbol_sequence(&mut self, 
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        match seq {
            SymbolSequence::Single(sym)       => self.derive_first_set_for_symbol(sym, first_set, break_set),
            SymbolSequence::Sequence(vec)     => self.derive_first_set_for_sequence(vec, first_set, break_set),
            SymbolSequence::Optional(sym)     => self.derive_first_set_for_optional(sym, first_set, break_set),
            SymbolSequence::Repeated(sym)     => self.derive_first_set_for_repeated(sym, first_set, break_set),
            SymbolSequence::Alternatives(vec) => self.derive_first_set_for_alternatives(vec, first_set, break_set),
        };
    }

    /**
     * Handles derivation of the First Set for a sequence of symbols.
     * This function breaks down the sequence into a set of symbols that could
     * possibly come at the start. That is - the First Set of the first symbol
     * in the sequence, unless that symbol can derive ε, in which case we need
     * the First Set of both the first and second symbols, etc.
     */
    fn derive_first_set_for_sequence(&mut self,
        seq:       &Vec<SymbolSequence<T, N>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
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

    /** 
     * Handles the derivation of the First Set for an optional sequence of symbols.
     * This works exactly the same as the derivation for a repeated sequence of symbols,
     * so we use a thunk, `first_set_repeated_optional_thunk` to handle it.
     */
    fn derive_first_set_for_optional(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    }

    /** 
     * Handles the derivation of the First Set for a repeated sequence of symbols.
     * This works exactly the same as the derivation for a repeated sequence of symbols,
     * so we use a thunk, `first_set_repeated_optional_thunk` to handle it.
     */
    fn derive_first_set_for_repeated(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    }
    
    /**
     * Handles the derivation of the First Set for a number of alternative
     * sequences of symbols. This just takes the First Sets of all the
     * alternatives and lumps them together.
     */
    fn derive_first_set_for_alternatives(&mut self,
        alts:      &Vec<SymbolSequence<T, N>>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        for alt in alts {
            self.derive_first_set_for_symbol_sequence(alt, first_set, break_set);
        }
    }

    /**
     * Helper method for derivation of First Sets for both optional and 
     * repeated sequences of symbols. This just takes the First Set of the
     * contained sequence and adds ε.
     */
    fn first_set_repeated_optional_thunk(&mut self,
        seq:       &SymbolSequence<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        self.derive_first_set_for_symbol_sequence(seq, first_set, break_set);
        first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    }

    /**
     * Helper method for derivation of First Sets for individual symbols.
     * If the provided symbol is terminal, it just gets added to the set.
     * If the provided symbol is nonterminal, its First Set gets added to the set.
     * If the provided symbol is empty, ε is added to the set.
     */
    fn derive_first_set_for_symbol(&mut self,
        symbol:    &Symbol<T, N>,
        first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
        break_set: &mut Vec<N>
    ) {
        match symbol {
            Symbol::Terminal(s)    => { 
                first_set.insert(PossiblyEmptyTerminalSymbol::Terminal(*s));
            },
            Symbol::Nonterminal(n) => { 
                if !break_set.contains(n) {
                    break_set.push(*n);
                    first_set.extend(self.get_first_set_for(n, break_set).clone());
                }
            },
            Symbol::Empty          => { 
                first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
            }
        }
    }

    /**
     * Serves as the connection between the bottom of the SymbolSequence tree back
     * to the top, used to derive or get First Sets for nonterminals that derive another nonterminal.
     */
    fn get_first_set_for(&mut self, non_terminal: &N, break_set: &mut Vec<N>) -> HashSet<PossiblyEmptyTerminalSymbol<T>> {
        if self.first_sets.contains_key(&non_terminal) {
            return self.first_sets.get(&non_terminal).unwrap().clone();
        } else {
            return self._derive_first_set(*non_terminal, break_set);
        }
    }
}
