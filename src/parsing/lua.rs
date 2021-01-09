use crate::parsing::*;
use crate::tokenisation::LuaToken;

pub fn get_lua_parser() {
    let production = Production::<LuaToken, LuaToken>::builder()
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
                    SymbolSequence::from_terminal(LuaToken::Else),
                    SymbolSequence::from_terminal(LuaToken::Varargs),
                ])
            ),
            SymbolSequence::from_nonterminal(LuaToken::End)
        ]))
        .with_handler(&reduce_production)
        .build()
        .unwrap();

    println!("{}", production);
}

fn reduce_production(symbols: Vec<Symbol<LuaToken, LuaToken>>) -> LuaToken {
    LuaToken::Colon
}
