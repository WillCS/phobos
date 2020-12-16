pub enum LuaNode {
    ChunkNode(LuaChunk),
    BlockNode(LuaBlock),
    StatNode(LuaStat),
    AttNameListNode(LuaAttNameList),
    Attrib(LuaAttrib),
    RetStat(LuaRetStat),
    Label(LuaLabel),
    FuncName(LuaFuncName),
    VarList(LuaVarList),
    Var(LuaVar),
    NameList(LuaNameList),
    ExpList(LuaExpList),
    Exp(LuaExp),
    PrefixExp(LuaPrefixExp),
    FunctionCall(LuaFunctionCall),
    Args(LuaArgs),
    FunctionDef(LuaFunctionDef),
    FuncBody(LuaFuncBody),
    ParList(LuaParList),
    TableConstructor(LuaTableConstructor),
    FieldList(LuaFieldList),
    Field(LuaField),
    FieldSep(LuaFieldSep)
}

pub struct LuaChunk {}

pub struct LuaBlock {}

pub struct LuaName {}

pub struct LuaNumber {}

pub struct LuaString {}

pub struct LuaBool {}

pub enum LuaStat {
    Semicolon,
    VarList(LuaVarList, LuaExpList),
    FunctionCall(LuaFunctionCall),
    Label(LuaLabel),
    Break,
    Goto(LuaName),
    Do(LuaBlock),
    While(LuaExp, LuaBlock),
    Repeat(LuaBlock, LuaExp),
    If(LuaExp, LuaBlock, Vec<(LuaExp, LuaBlock)>, Option<LuaBlock>),
    For(LuaName, LuaExp, LuaExp, Option<LuaExp>, LuaBlock),
    ForIn(LuaNameList, LuaExpList, LuaBlock),
    Function(LuaFuncName, LuaFuncBody),
    LocalFunction(LuaFuncName, LuaFuncBody),
    LocalAttNameList(LuaAttNameList, Option<LuaExpList>)
}

pub struct LuaAttNameList {}

pub struct LuaAttrib {}

pub struct LuaRetStat {}

pub struct LuaLabel {}

pub struct LuaFuncName {}

pub struct LuaVarList {}

pub enum LuaVar {
    Name(LuaName),
    Index(LuaPrefixExp, LuaExp),
    Field(LuaPrefixExp, LuaName)
}

pub struct LuaNameList {}

pub struct LuaExpList {}

pub enum LuaExp {
    OrOp(Box<LuaExp>, LuaExp2),
    Other(LuaExp2)
}

pub enum LuaExp2 {
    AndOp(Box<LuaExp2>, LuaExp3),
    Other(LuaExp3)
}

pub enum LuaExp3 {
    LessOp(Box<LuaExp3>, LuaExp4),
    GreaterOp(Box<LuaExp3>, LuaExp4),
    LessEqOp(Box<LuaExp3>, LuaExp4),
    GreaterEqOp(Box<LuaExp3>, LuaExp4),
    NotEqOp(Box<LuaExp3>, LuaExp4),
    EqOp(Box<LuaExp3>, LuaExp4),
    Other(LuaExp4)
}

pub enum LuaExp4 {
    BinOrOp(Box<LuaExp4>, LuaExp5),
    Other(LuaExp5)
}

pub enum LuaExp5 {
    BinNotOp(Box<LuaExp5>, LuaExp6),
    Other(LuaExp6)
}

pub enum LuaExp6 {
    BinAndOp(Box<LuaExp6>, LuaExp7),
    Other(LuaExp7)
}

pub enum LuaExp7 {
    LeftShiftOp(Box<LuaExp7>, LuaExp8),
    RightShoftOp(Box<LuaExp7>, LuaExp8),
    Other(LuaExp8)
}

pub enum LuaExp8 {
    ConcatOp(LuaExp9, Box<LuaExp8>), // Right associative
    Other(LuaExp9)
}

pub enum LuaExp9 {
    AddOp(Box<LuaExp9>, LuaExp10),
    MinOp(Box<LuaExp9>, LuaExp10),
    Other(LuaExp10)
}

pub enum LuaExp10 {
    MulOp(Box<LuaExp10>, LuaExp11),
    DivOp(Box<LuaExp10>, LuaExp11),
    FloorDivOp(Box<LuaExp10>, LuaExp11),
    ModOp(Box<LuaExp10>, LuaExp11),
    Other(LuaExp11)
}

pub enum LuaExp11 {
    NotOp(LuaExp12),
    LenOp(LuaExp12),
    NegOp(LuaExp12),
    BinNotOp(LuaExp12),
    Other(LuaExp12)
}

pub enum LuaExp12 {
    Power(LuaExp13, Box<LuaExp12>), // Right associative
    Other(LuaExp13)
}

pub enum LuaExp13 {
    Nil,
    Boolean(bool),
    NumberExp(f64),
    StringExp(String),
    Varargs,
    FunctionDef(LuaFunctionDef),
    PrefixExp(LuaPrefixExp),
    TableConstructor(LuaTableConstructor)
}

pub enum LuaPrefixExp {
    Var(Box<LuaVar>),
    FunctionCall(Box<LuaFunctionCall>),
    BracketedExp(Box<LuaExp>)
}

pub enum LuaFunctionCall {
    Function(LuaPrefixExp, LuaArgs),
    Method(LuaPrefixExp, LuaName, LuaArgs)
}

pub enum LuaArgs {
    ExpList(LuaExpList),
    TableConstructor(LuaTableConstructor),
    LiteralString(LuaString)
}

pub struct LuaFunctionDef {}

pub struct LuaFuncBody {}

pub enum LuaParList {
    JustNames(LuaNameList),
    NamesAndVarargs(LuaNameList),
    JustVarargs
}

pub struct LuaTableConstructor {}

pub struct LuaFieldList {}

pub enum LuaField {
    FieldDefined(LuaExp, LuaExp),
    NameDefined(LuaName, LuaExp),
    ExpDefined(LuaExp)
}

pub struct LuaFieldSep {}
