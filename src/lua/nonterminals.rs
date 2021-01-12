use std::fmt::{Display, Formatter};
use std::mem::discriminant;

use enum_iterator::IntoEnumIterator;

use crate::parsing::NonterminalSymbol;
use crate::lua::syntax_tree::LuaNode;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, IntoEnumIterator)]
pub enum LuaNonterminal {
    Chunk,
    Block,
    Stat,
    AttNameList,
    Attrib,
    RetStat,
    Label,
    FuncName,
    VarList,
    Var,
    NameList,
    ExpList,
    Exp,
    Exp2,
    Exp3,
    Exp4,
    Exp5,
    Exp6,
    Exp7,
    Exp8,
    Exp9,
    Exp10,
    Exp11,
    Exp12,
    Exp13,
    PrefixExp,
    FunctionCall,
    Args,
    FunctionDef,
    FuncBody,
    ParList,
    TableConstructor,
    FieldList,
    Field,
    FieldSep
}

impl Display for LuaNonterminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.get_name())
    }
}

impl NonterminalSymbol for LuaNonterminal {
    type NodeType = LuaNode;
    
    fn get_name(&self) -> &'static str {
        match self {
            LuaNonterminal::Chunk            => "chunk",
            LuaNonterminal::Block            => "block",
            LuaNonterminal::Stat             => "stat",
            LuaNonterminal::AttNameList      => "attnamelist",
            LuaNonterminal::Attrib           => "attrib",
            LuaNonterminal::RetStat          => "retstat",
            LuaNonterminal::Label            => "label",
            LuaNonterminal::FuncName         => "funcname",
            LuaNonterminal::VarList          => "varlist",
            LuaNonterminal::Var              => "var",
            LuaNonterminal::NameList         => "namelist",
            LuaNonterminal::ExpList          => "explist",
            LuaNonterminal::Exp              => "exp",
            LuaNonterminal::Exp2             => "exp2",
            LuaNonterminal::Exp3             => "exp3",
            LuaNonterminal::Exp4             => "exp4",
            LuaNonterminal::Exp5             => "exp5",
            LuaNonterminal::Exp6             => "exp6",
            LuaNonterminal::Exp7             => "exp7",
            LuaNonterminal::Exp8             => "exp8",
            LuaNonterminal::Exp9             => "exp9",
            LuaNonterminal::Exp10            => "exp10",
            LuaNonterminal::Exp11            => "exp11",
            LuaNonterminal::Exp12            => "exp12",
            LuaNonterminal::Exp13            => "exp13",
            LuaNonterminal::PrefixExp        => "prefixexp",
            LuaNonterminal::FunctionCall     => "functioncall",
            LuaNonterminal::Args             => "args",
            LuaNonterminal::FunctionDef      => "functiondef",
            LuaNonterminal::FuncBody         => "funcbody",
            LuaNonterminal::ParList          => "parlist",
            LuaNonterminal::TableConstructor => "tableconstructor",
            LuaNonterminal::FieldList        => "fieldlist",
            LuaNonterminal::Field            => "field",
            LuaNonterminal::FieldSep         => "fieldsep"
        }
    }

    fn same_symbol(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
