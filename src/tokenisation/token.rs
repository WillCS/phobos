use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Token<T> where T: Display {
    pub token_type: T,
    pub location:   Location
}

impl<T: Display> Display for Token<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} at {}", self.token_type, self.location)
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
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
    Error(String),
    Identifier(String),
    StringLiteral(String),
    NumberLiteral(f64)
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TokenType::Identifier(name)     => write!(f, "{}", name),
            TokenType::StringLiteral(value) => write!(f, "'{}'", value),
            TokenType::NumberLiteral(value) => write!(f, "'{}'", value),
            _                               => write!(f, "{:?}", self)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub col:  usize
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}
