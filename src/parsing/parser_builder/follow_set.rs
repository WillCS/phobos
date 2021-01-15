use std::collections::{HashMap, HashSet};

use crate::parsing::{ParserBuilder, TerminalSymbol, NonterminalSymbol, SymbolSequence, PossiblyEndOfFileTerminalSymbol, PossiblyEmptyTerminalSymbol, Symbol, Production};

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
    pub(super) fn derive_follow_sets(&mut self, start_symbol: N) {
        let mut follow_sets = HashMap::new();
        N::into_enum_iter()
            .for_each(|nonterminal| {
                self.follow_sets.insert(nonterminal, HashSet::new());
            });

        self.follow_sets[&start_symbol].insert(PossiblyEndOfFileTerminalSymbol::EndOfFile);

        for p in &self.productions[..] {
            ParserBuilder::_derive_follow_set(p, &mut follow_sets, &self.first_sets);
        }

        self.follow_sets = follow_sets;
    }

    fn _derive_follow_set(
        production:  &Production<T, N>,
        follow_sets: &mut HashMap<N, HashSet<PossiblyEndOfFileTerminalSymbol<T>>>,
        first_sets:  &HashMap<N, HashSet<PossiblyEmptyTerminalSymbol<T>>>
    ) {
        match production.consumed_symbols {
            SymbolSequence::Single(sym) => 
                ParserBuilder::update_follow_set_from_single(&production.produced_symbol, &sym, follow_sets, first_sets),
            SymbolSequence::Sequence(seq) =>
        }
    }

    fn update_follow_set_from_single(
        nonterminal: &N,
        symbol:      &Symbol<T, N>,
        follow_sets: &mut HashMap<N, HashSet<PossiblyEndOfFileTerminalSymbol<T>>>,
        first_sets:  &HashMap<N, HashSet<PossiblyEmptyTerminalSymbol<T>>>
    ) {
        match symbol {
            Symbol::Terminal(_) | Symbol::Empty => { },
            Symbol::Nonterminal(n) => {
                let first_set_of_n: HashSet<PossiblyEndOfFileTerminalSymbol<T>> = first_sets[&n]
                    .iter()
                    .filter_map(|terminal| match terminal {
                        PossiblyEmptyTerminalSymbol::Terminal(t) => Some(PossiblyEndOfFileTerminalSymbol::Terminal(*t)),
                        PossiblyEmptyTerminalSymbol::Empty       => None
                    })
                    .collect();
                
                follow_sets[nonterminal].extend(first_set_of_n)
            }
        }
    }
}
    // /**
    //  * Derives and returns the first set for the specified nonterminal symbol.
    //  */
    // fn derive_first_set_for(&mut self,
    //     non_terminal: U, 
    //     break_set:    &mut Vec<U>
    // ) -> HashSet<PossiblyEmptyTerminalSymbol<T>> {
    //     let mut first_set = HashSet::new();
    //     let mut relevant_productions = Vec::new();

    //     for p in &self.productions {
    //         if p.produces(non_terminal) {
    //             relevant_productions.push(p.consumed_symbols.clone());
    //         }
    //     }

    //     for p in relevant_productions {
    //         self.derive_first_set_for_symbol_sequence(&p, &mut first_set, break_set);
    //     }

    //     return first_set;
    // }

    // /**
    //  * Delegates how the First Set of a SymbolSequence should be derived based 
    //  * on what kind of SymbolSequence it is.
    //  */
    // fn derive_first_set_for_symbol_sequence(&mut self, 
    //     seq:       &SymbolSequence<T, U>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     match seq {
    //         SymbolSequence::Single(s)       => self.derive_first_set_for_symbol(s, first_set, break_set),
    //         SymbolSequence::Sequence(v)     => self.derive_first_set_for_sequence(v, first_set, break_set),
    //         SymbolSequence::Optional(s)     => self.derive_first_set_for_optional(&s, first_set, break_set),
    //         SymbolSequence::Repeated(s)     => self.derive_first_set_for_repeated(&s, first_set, break_set),
    //         SymbolSequence::Alternatives(v) => self.derive_first_set_for_alternatives(&v, first_set, break_set),
    //     };
    // }

    // /**
    //  * Handles derivation of the First Set for a sequence of symbols.
    //  * This function breaks down the sequence into a set of symbols that could
    //  * possibly come at the start. That is - the First Set of the first symbol
    //  * in the sequence, unless that symbol can derive ε, in which case we need
    //  * the First Set of both the first and second symbols, etc.
    //  */
    // fn derive_first_set_for_sequence(&mut self,
    //     seq:       &Vec<SymbolSequence<T, U>>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     for s in seq {
    //         let mut first_set_of_s = HashSet::new();
    //         self.derive_first_set_for_symbol_sequence(&s, &mut first_set_of_s, break_set);

    //         let reduces_to_empty = first_set_of_s.contains(&PossiblyEmptyTerminalSymbol::Empty);
    //         first_set.extend(first_set_of_s);

    //         if !reduces_to_empty {
    //             break;
    //         }
    //     }
    // }

    // /** 
    //  * Handles the derivation of the First Set for an optional sequence of symbols.
    //  * This works exactly the same as the derivation for a repeated sequence of symbols,
    //  * so we use a thunk, `first_set_repeated_optional_thunk` to handle it.
    //  */
    // fn derive_first_set_for_optional(&mut self,
    //     seq:       &SymbolSequence<T, U>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    // }

    // /** 
    //  * Handles the derivation of the First Set for a repeated sequence of symbols.
    //  * This works exactly the same as the derivation for a repeated sequence of symbols,
    //  * so we use a thunk, `first_set_repeated_optional_thunk` to handle it.
    //  */
    // fn derive_first_set_for_repeated(&mut self,
    //     seq:       &SymbolSequence<T, U>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     self.first_set_repeated_optional_thunk(seq, first_set, break_set);
    // }
    
    // /**
    //  * Handles the derivation of the First Set for a number of alternative
    //  * sequences of symbols. This just takes the First Sets of all the
    //  * alternatives and lumps them together.
    //  */
    // fn derive_first_set_for_alternatives(&mut self,
    //     alts:      &Vec<SymbolSequence<T, U>>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     for alt in alts {
    //         self.derive_first_set_for_symbol_sequence(alt, first_set, break_set);
    //     }
    // }

    // /**
    //  * Helper method for derivation of First Sets for both optional and 
    //  * repeated sequences of symbols. This just takes the First Set of the
    //  * contained sequence and adds ε.
    //  */
    // fn first_set_repeated_optional_thunk(&mut self,
    //     seq:       &SymbolSequence<T, U>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     self.derive_first_set_for_symbol_sequence(seq, first_set, break_set);
    //     first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    // }

    // /**
    //  * Helper method for derivation of First Sets for individual symbols.
    //  * If the provided symbol is terminal, it just gets added to the set.
    //  * If the provided symbol is nonterminal, its First Set gets added to the set.
    //  * If the provided symbol is empty, ε is added to the set.
    //  */
    // fn derive_first_set_for_symbol(&mut self,
    //     symbol:    &Symbol<T, U>,
    //     first_set: &mut HashSet<PossiblyEmptyTerminalSymbol<T>>,
    //     break_set: &mut Vec<U>
    // ) {
    //     match symbol {
    //         Symbol::Terminal(s)    => { 
    //             first_set.insert(PossiblyEmptyTerminalSymbol::Terminal(*s));
    //         },
    //         Symbol::Nonterminal(n) => { 
    //             if !break_set.contains(n) {
    //                 break_set.push(*n);
    //                 first_set.extend(self.get_first_set_for(n, break_set).clone());
    //             }
    //         },
    //         Symbol::Empty          => { 
    //             first_set.insert(PossiblyEmptyTerminalSymbol::Empty);
    //         }
    //     }
    // }

    // /**
    //  * Serves as the connection between the bottom of the SymbolSequence tree back
    //  * to the top, used to derive or get First Sets for nonterminals that derive another nonterminal.
    //  */
    // fn get_first_set_for(&mut self, non_terminal: &U, break_set: &mut Vec<U>) -> HashSet<PossiblyEmptyTerminalSymbol<T>> {
    //     if self.first_sets.contains_key(&non_terminal) {
    //         return self.first_sets.get(&non_terminal).unwrap().clone();
    //     } else {
    //         return self.derive_first_set_for(*non_terminal, break_set);
    //     }
    // }
// }
