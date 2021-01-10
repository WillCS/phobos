use std::fmt::{Display, Formatter};

use crate::tokenisation::TokenData;
use crate::parsing::TerminalSymbol;

pub enum LuaTokenData {
    Error(String),
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String)
}

impl TokenData for LuaTokenData {
    
}

impl Display for LuaTokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LuaTokenData::Error(val)           => write!(f, "{}",   val),
            LuaTokenData::Identifier(name)     => write!(f, "{}",   name),
            LuaTokenData::StringLiteral(value) => write!(f, "'{}'", value),
            LuaTokenData::NumberLiteral(value) => write!(f, "'{}'", value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LuaTerminal {
    End,
    Do,
    While,
    Repeat,
    Until,
    If,
    In,
    Then,
    Elseif,
    Else,
    For,
    Function,
    Local,
    Return,
    Break,
    True,
    False,
    Nil,
    And,
    Or,
    Not,
    Goto,
    Equals,
    DoubleEquals,
    Dot,
    Colon,
    DoubleColon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNeg,
    Varargs,
    Semicolon,
    Plus,
    Minus,
    Multiply,
    Divide,
    FloorDivide,
    Power,
    Modulo,
    Concat,
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEq,
    NotEq,
    Length,
    Comment,
    EndOfFile,
    Error,
    Identifier,
    StringLiteral,
    NumberLiteral
}

impl TerminalSymbol for LuaTerminal {
    type DataType = LuaTokenData;

    fn get_name(&self) -> &'static str {
        match self {
            LuaTerminal::End              => "end",
            LuaTerminal::Do               => "do",
            LuaTerminal::While            => "while",
            LuaTerminal::Repeat           => "repeat",
            LuaTerminal::Until            => "until",
            LuaTerminal::If               => "if",
            LuaTerminal::In               => "in",
            LuaTerminal::Then             => "then",
            LuaTerminal::Elseif           => "elseif",
            LuaTerminal::Else             => "else",
            LuaTerminal::For              => "for",
            LuaTerminal::Function         => "function",
            LuaTerminal::Local            => "local",
            LuaTerminal::Return           => "return",
            LuaTerminal::Break            => "break",
            LuaTerminal::True             => "true",
            LuaTerminal::False            => "false",
            LuaTerminal::Nil              => "nil",
            LuaTerminal::And              => "and",
            LuaTerminal::Or               => "or",
            LuaTerminal::Not              => "not",
            LuaTerminal::Goto             => "goto",
            LuaTerminal::Equals           => "`=`",
            LuaTerminal::DoubleEquals     => "`==`",
            LuaTerminal::Dot              => "`.`",
            LuaTerminal::Colon            => "`:`",
            LuaTerminal::DoubleColon      => "`::`",
            LuaTerminal::Comma            => "`,`",
            LuaTerminal::LeftBracket      => "`[`",
            LuaTerminal::RightBracket     => "`]`",
            LuaTerminal::LeftParenthesis  => "`(`",
            LuaTerminal::RightParenthesis => "`)`",
            LuaTerminal::LeftBrace        => "`{`",
            LuaTerminal::RightBrace       => "`}`",
            LuaTerminal::LeftShift        => "`<<`",
            LuaTerminal::RightShift       => "`>>`",
            LuaTerminal::BitwiseAnd       => "`&`",
            LuaTerminal::BitwiseOr        => "`|`",
            LuaTerminal::BitwiseNeg       => "`~`",
            LuaTerminal::Varargs          => "`...`",
            LuaTerminal::Semicolon        => "`;`",
            LuaTerminal::Plus             => "`+`",
            LuaTerminal::Minus            => "`-`",
            LuaTerminal::Multiply         => "`*`",
            LuaTerminal::Divide           => "`/`",
            LuaTerminal::FloorDivide      => "`//`",
            LuaTerminal::Power            => "`^`",
            LuaTerminal::Modulo           => "`%`",
            LuaTerminal::Concat           => "`..`",
            LuaTerminal::LessThan         => "`<`",
            LuaTerminal::LessEq           => "`<=`",
            LuaTerminal::GreaterThan      => "`>`",
            LuaTerminal::GreaterEq        => "`>=`",
            LuaTerminal::NotEq            => "`~=`",
            LuaTerminal::Length           => "`#`",
            LuaTerminal::Comment          => "Comment",
            LuaTerminal::EndOfFile        => "<eof>",
            LuaTerminal::Error            => "Error",
            LuaTerminal::Identifier       => "Name",
            LuaTerminal::StringLiteral    => "LiteralString",
            LuaTerminal::NumberLiteral    => "Numeral"
        }
    }
}

impl Display for LuaTerminal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
