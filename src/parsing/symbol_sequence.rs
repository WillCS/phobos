use std::collections::HashSet;
use crate::parsing::{Symbol, TerminalSymbol, NonterminalSymbol};

use std::fmt::{Display, Formatter, Write};
use std::vec::Vec;

/** Symbol used for construction of productions for the parser. */
#[derive(Clone)]
pub enum SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    Single(Symbol<T, U>),
    Sequence(Vec<SymbolSequence<T, U>>),
    Optional(Box<SymbolSequence<T, U>>),
    Repeated(Box<SymbolSequence<T, U>>),
    Alternatives(Vec<SymbolSequence<T, U>>),
}

impl<T, U> SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    pub fn from_terminal(terminal: T) -> SymbolSequence<T, U> {
        SymbolSequence::Single(Symbol::Terminal(terminal))
    }

    pub fn from_nonterminal(nonterminal: U) -> SymbolSequence<T, U> {
        SymbolSequence::Single(Symbol::Nonterminal(nonterminal))
    }

    pub fn either(
        opt1: SymbolSequence<T, U>,
        opt2: SymbolSequence<T, U>
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Alternatives(vec![opt1, opt2])
    }

    pub fn many(
        seq: SymbolSequence<T, U>,
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Repeated(Box::new(seq))
    }

    pub fn maybe(
        seq: SymbolSequence<T, U>,
    ) -> SymbolSequence<T, U> {
        SymbolSequence::Optional(Box::new(seq))
    }
}

impl<T, U> SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    // pub fn get_starting_symbols(&self) -> HashSet<Symbol<T, U>> {
    //     let mut starting_symbols = HashSet::new();
    //     match self {
    //         SymbolSequence::Single(sym)   => { starting_symbols.insert(*sym); },
    //         SymbolSequence::Sequence(seq) => {
    //             for sub in seq {
    //                 let sub_starting_symbols = sub.get_starting_symbols();
    //                 let contains_empty = sub_starting_symbols.contains(&Symbol::Empty);
    //                 starting_symbols.extend(&sub_starting_symbols);

    //                 if !contains_empty {
    //                     break;
    //                 }
    //             }
    //         },
    //         SymbolSequence::Optional(seq) | SymbolSequence::Repeated(seq) => {
    //             let seq_starting_symbols = seq.get_starting_symbols();
    //             starting_symbols.extend(&seq_starting_symbols);
    //             starting_symbols.insert(Symbol::Empty);
    //         },
    //         SymbolSequence::Alternatives(alts) => {
    //             for alt in alts {
    //                 let alt_starting_symbols = alt.get_starting_symbols();
    //                 starting_symbols.extend(alt_starting_symbols);
    //             }
    //         }
    //     };
    //     return starting_symbols;
    // }

    pub fn collect_dependencies(&self) -> HashSet<Symbol<T, U>> {
        let mut symbols = HashSet::new();

        match self {
            SymbolSequence::Single(sym)   => { symbols.insert(*sym); }
            SymbolSequence::Sequence(seq) => {
                for sub in seq {
                    let sub_symbols = sub.collect_dependencies();
                    symbols.extend(sub_symbols);
                }
            },
            SymbolSequence::Optional(seq) | SymbolSequence::Repeated(seq) => {
                let seq_symbols = seq.collect_dependencies();
                symbols.extend(seq_symbols);
            },
            SymbolSequence::Alternatives(alts) => {
                for alt in alts {
                    let alt_symbols = alt.collect_dependencies();
                    symbols.extend(alt_symbols);
                }
            }
        }

        return symbols;
    }
}

impl<T, U> From<Symbol<T, U>> for SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn from(symbol: Symbol<T, U>) -> SymbolSequence<T, U> {
        SymbolSequence::Single(symbol)
    }
}

impl<T, U> Display for SymbolSequence<T, U> where T: TerminalSymbol, U: NonterminalSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SymbolSequence::Single(s)   => write!(f, "{}", s),
            SymbolSequence::Sequence(v) => {
                let mut output = "".to_string();
                let mut i = 1;
                for symbol in v {
                    write!(output, "{}", symbol).unwrap();
                    if i != v.len() {
                        write!(output, " ").unwrap();
                    }

                    i += 1;
                }

                write!(f, "{}", output)
            },
            SymbolSequence::Optional(s) => write!(f, "[ {} ]", s),
            SymbolSequence::Repeated(s) => write!(f, "{{ {} }}", s),
            SymbolSequence::Alternatives(v) => {
                let mut output = "".to_string();
                let mut i = 1;
                for symbol in v {
                    write!(output, "{}", symbol).unwrap();
                    if i != v.len() {
                        write!(output, " | ").unwrap();
                    }

                    i += 1;
                }

                write!(f, "{}", output)
            }
        }
    }
}
