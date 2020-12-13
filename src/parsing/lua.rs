use crate::parsing::*;
use crate::tokenisation::LuaToken;

pub fn get_lua_parser() {
    Production::<LuaToken, LuaToken>::builder()
        .producing(LuaToken::Varargs)
        .from(SymbolSequence::Sequence(vec![
            SymbolSequence::from_nonterminal(LuaToken::If),
            SymbolSequence::from_terminal(LuaToken::Varargs),
            SymbolSequence::from_nonterminal(LuaToken::Then),
            SymbolSequence::from_terminal(LuaToken::Varargs),
            SymbolSequence::many(
                SymbolSequence::Sequence(vec![
                    SymbolSequence::from_nonterminal(LuaToken::Elseif),
                    SymbolSequence::from_terminal(LuaToken::Varargs),
                    SymbolSequence::from_nonterminal(LuaToken::Then),
                    SymbolSequence::from_terminal(LuaToken::Varargs),
                ])
            ),
            SymbolSequence::maybe(
                SymbolSequence::Sequence(vec![
                    SymbolSequence::from(Symbol::Terminal(LuaToken::Else)),
                    SymbolSequence::from(Symbol::Nonterminal(LuaToken::Varargs)),
                ])
            ),
            SymbolSequence::from_nonterminal(LuaToken::End)
        ]))
        .build();
}
