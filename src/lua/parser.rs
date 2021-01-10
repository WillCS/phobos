use crate::lua::{LuaTerminal, LuaNonterminal};
use crate::parsing::*;

macro_rules! t {
    ($t:ident) => { SymbolSequence::from_terminal(LuaTerminal::$t) };
}

macro_rules! n {
    ($t:ident) => { SymbolSequence::from_nonterminal(LuaNonterminal::$t) };
}

macro_rules! seq {
    ($($e:expr),+) => { SymbolSequence::Sequence(vec![$($e),+]) };
}

macro_rules! one_of {
    ($($e:expr),+) => { SymbolSequence::Alternatives(vec![$($e),+]) };
}

macro_rules! many {
    ($e:expr) => { SymbolSequence::many($e) };
}

macro_rules! maybe {
    ($e:expr) => { SymbolSequence::maybe($e) };
}

pub fn get_lua_parser() {
    let mut productions: Vec<Production<LuaTerminal, LuaNonterminal>> = Vec::new();

    productions.push(Production::builder()
        .producing(LuaNonterminal::Chunk)
        .from(n!(Block))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Block)
        .from(seq!(
            many!(n!(Stat)),
            maybe!(n!(RetStat))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Stat)
        .from(one_of!(
            t!(Semicolon),
            seq!(n!(VarList), t!(Equals), n!(ExpList)),
            n!(FunctionCall),
            n!(Label),
            t!(Break),
            seq!(t!(Goto), t!(Identifier)),
            seq!(t!(Do), n!(Block), t!(End)),
            seq!(t!(While), n!(Exp), t!(Do), n!(Block), t!(End)),
            seq!(t!(Repeat), n!(Block), t!(Until), n!(Exp)),
            seq!(
                t!(If),
                n!(Exp),
                t!(Then),
                n!(Block),
                many!(seq!(
                    t!(Elseif),
                    n!(Exp),
                    t!(Then),
                    n!(Block))),
                maybe!(seq!(
                    t!(Else),
                    n!(Block))),
                t!(End)),
            seq!(
                t!(For),
                t!(Identifier),
                t!(Equals),
                n!(Exp),
                t!(Comma),
                n!(Exp),
                maybe!(seq!(
                    t!(Comma),
                    n!(Exp))),
                t!(Do),
                n!(Block),
                t!(End)),
            seq!(t!(For), n!(NameList), t!(In), n!(ExpList), t!(Do), n!(Block), t!(End)),
            seq!(t!(Function), n!(FuncName), n!(FuncBody)),
            seq!(t!(Local), t!(Function), t!(Identifier), n!(FuncBody)),
            seq!(
                t!(Local),
                n!(AttNameList),
                maybe!(seq!(
                    t!(Equals),
                    n!(ExpList))))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());
    
    productions.push(Production::builder()
        .producing(LuaNonterminal::AttNameList)
        .from(seq!(
            t!(Identifier),
            n!(Attrib),
            many!(seq!(
                t!(Comma),
                t!(Identifier),
                n!(Attrib)))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());
    
    productions.push(Production::builder()
        .producing(LuaNonterminal::Attrib)
        .from(maybe!(seq!(
            t!(LessThan),
            t!(Identifier),
            t!(GreaterThan))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());
    
    productions.push(Production::builder()
        .producing(LuaNonterminal::RetStat)
        .from(seq!(
            t!(Return),
            maybe!(n!(ExpList)),
            maybe!(t!(Semicolon))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Label)
        .from(seq!(
            t!(DoubleColon),
            t!(Identifier),
            t!(DoubleColon)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FuncName)
        .from(seq!(
            t!(Identifier),
            many!(seq!(
                t!(Dot),
                t!(Identifier))),
            maybe!(seq!(
                t!(Colon),
                t!(Identifier)))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::VarList)
        .from(seq!(
            n!(Var),
            many!(seq!(
                t!(Comma),
                n!(Var)))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Var)
        .from(one_of!(
            t!(Identifier),
            seq!(
                n!(PrefixExp),
                t!(LeftBracket),
                n!(Exp),
                t!(RightBracket)),
            seq!(
                n!(PrefixExp),
                t!(Dot),
                t!(Identifier)
            )))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::NameList)
        .from(seq!(
            t!(Identifier),
            many!(seq!(
                t!(Comma),
                t!(Identifier)))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::ExpList)
        .from(seq!(
            n!(Exp),
            many!(seq!(
                t!(Comma),
                n!(Exp)))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp)
        .from(one_of!(
            seq!(
                n!(Exp),
                t!(Or),
                n!(Exp2)),
            n!(Exp2)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp2)
        .from(one_of!(
            seq!(
                n!(Exp2),
                t!(And),
                n!(Exp3)),
            n!(Exp3)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp3)
        .from(one_of!(
            seq!(
                n!(Exp3),
                t!(LessThan),
                n!(Exp4)),
            seq!(
                n!(Exp3),
                t!(GreaterThan),
                n!(Exp4)),
            seq!(
                n!(Exp3),
                t!(LessEq),
                n!(Exp4)),
            seq!(
                n!(Exp3),
                t!(GreaterEq),
                n!(Exp4)),
            seq!(
                n!(Exp3),
                t!(NotEq),
                n!(Exp4)),
            seq!(
                n!(Exp3),
                t!(DoubleEquals),
                n!(Exp4)),
            n!(Exp4)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp4)
        .from(one_of!(
            seq!(
                n!(Exp4),
                t!(BitwiseOr),
                n!(Exp5)),
            n!(Exp5)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp5)
        .from(one_of!(
            seq!(
                n!(Exp5),
                t!(BitwiseNeg),
                n!(Exp6)),
            n!(Exp6)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp6)
        .from(one_of!(
            seq!(
                n!(Exp6),
                t!(BitwiseAnd),
                n!(Exp7)),
            n!(Exp7)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp7)
        .from(one_of!(
            seq!(
                n!(Exp7),
                t!(LeftShift),
                n!(Exp8)),
            seq!(
                n!(Exp7),
                t!(RightShift),
                n!(Exp8)),
            n!(Exp8)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp8)
        .from(one_of!(
            seq!(
                n!(Exp9),
                t!(Concat),
                n!(Exp8)),
            n!(Exp9)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp9)
        .from(one_of!(
            seq!(
                n!(Exp9),
                t!(Plus),
                n!(Exp10)),
            seq!(
                n!(Exp9),
                t!(Minus),
                n!(Exp10)),
            n!(Exp10)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp10)
        .from(one_of!(
            seq!(
                n!(Exp10),
                t!(Multiply),
                n!(Exp11)),
            seq!(
                n!(Exp10),
                t!(Divide),
                n!(Exp11)),
            seq!(
                n!(Exp10),
                t!(FloorDivide),
                n!(Exp11)),
            seq!(
                n!(Exp10),
                t!(Modulo),
                n!(Exp11)),
            n!(Exp11)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp11)
        .from(one_of!(
            seq!(
                t!(Not),
                n!(Exp12)),
            seq!(
                t!(Length),
                n!(Exp12)),
            seq!(
                t!(Minus),
                n!(Exp12)),
            seq!(
                t!(BitwiseNeg),
                n!(Exp12)),
            n!(Exp12)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp12)
        .from(one_of!(
            seq!(
                n!(Exp13),
                t!(Power),
                n!(Exp12)),
            n!(Exp13)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Exp13)
        .from(one_of!(
            t!(Nil),
            t!(False),
            t!(True),
            t!(NumberLiteral),
            t!(StringLiteral),
            t!(Varargs),
            n!(FunctionDef),
            n!(PrefixExp),
            n!(TableConstructor)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::PrefixExp)
        .from(one_of!(
            n!(Var),
            n!(FunctionCall),
            seq!(
                t!(LeftParenthesis),
                n!(Exp),
                t!(RightParenthesis))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FunctionCall)
        .from(one_of!(
            seq!(
                n!(PrefixExp),
                n!(Args)),
            seq!(
                n!(PrefixExp),
                t!(Colon),
                t!(Identifier),
                n!(Args))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Args)
        .from(one_of!(
            seq!(
                t!(LeftParenthesis),
                maybe!(n!(ExpList)),
                t!(RightParenthesis)),
            n!(TableConstructor),
            t!(StringLiteral)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FunctionDef)
        .from(seq!(
            t!(Function),
            n!(FuncBody)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FuncBody)
        .from(seq!(
            t!(LeftParenthesis),
            maybe!(n!(ParList)),
            t!(RightParenthesis),
            n!(Block),
            t!(End)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::ParList)
        .from(one_of!(
            seq!(
                n!(NameList),
                maybe!(seq!(
                    t!(Comma),
                    t!(Varargs)))),
            t!(Varargs)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::TableConstructor)
        .from(seq!(
            t!(LeftBrace),
            maybe!(n!(FieldList)),
            t!(RightBrace)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FieldList)
        .from(seq!(
            n!(Field),
            many!(seq!(
                n!(FieldSep),
                n!(Field))),
            maybe!(n!(FieldSep))))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::Field)
        .from(one_of!(
            seq!(
                t!(LeftBracket),
                n!(Exp),
                t!(RightBracket),
                t!(Equals),
                n!(Exp)),
            seq!(
                t!(Identifier),
                t!(Equals),
                n!(Exp)),
            n!(Exp)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    productions.push(Production::builder()
        .producing(LuaNonterminal::FieldSep)
        .from(one_of!(
            t!(Comma),
            t!(Semicolon)))
        .with_handler(&reduce_production)
        .build()
        .unwrap());

    for p in productions {
        println!("{}", p);
    }

    let parser_builder = ParserBuilder::new()
        .with_productions(&mut productions)
        .with_start_symbol(LuaNonterminal::Chunk)
        .with_empty_symbol(LuaTerminal::Empty)
        .build();
    
}

fn reduce_production(symbols: Vec<Symbol<LuaTerminal, LuaNonterminal>>) -> LuaNonterminal {
    LuaNonterminal::Exp
}
