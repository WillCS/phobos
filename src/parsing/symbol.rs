use std::vec::Vec;

/** Symbol used internally within the parser. */
#[derive(PartialEq, Eq, Hash)]
pub enum Symbol<T, U> {
    Terminal(T),
    Nonterminal(U)
}

impl<T, U> Symbol<T, U> {
    pub fn from_terminal(terminal: T) -> Symbol<T, U> {
        Symbol::Terminal(terminal)
    }

    pub fn from_nonterminal(nonterminal: U) -> Symbol<T, U> {
        Symbol::Nonterminal(nonterminal)
    }
}

/** Symbol used for construction of productions for the parser. */
pub enum SymbolSequence<T, U> {
    Single(Symbol<T, U>),
    Sequence(Vec<SymbolSequence<T, U>>),
    Optional(Box<SymbolSequence<T, U>>),
    Repeated(Box<SymbolSequence<T, U>>),
    Alternatives(Vec<SymbolSequence<T, U>>)
}

impl<T, U> SymbolSequence<T, U> {
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

impl<T, U> From<Symbol<T, U>> for SymbolSequence<T, U> {
    fn from(symbol: Symbol<T, U>) -> SymbolSequence<T, U> {
        SymbolSequence::Single(symbol)
    }
}