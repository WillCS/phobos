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
    End,                   // r"\bend\b"
    Do,                    // r"\bdo\b"
    While,                 // r"\bwhile\b"
    Repeat,                // r"\brepeat\b"
    Until,                 // r"\buntil\b"
    If,                    // r"\bif\b"
    In,                    // r"\bin\b"
    Then,                  // r"\bthen\b"
    Elseif,                // r"\belseif\b"
    Else,                  // r"\belse\b"
    For,                   // r"\bfor\b"
    Function,              // r"\bfunction\b"
    Local,                 // r"\blocal\b"
    Return,                // r"\breturn\b"
    Break,                 // r"\bbreak\b"
    True,                  // r"\btrue\b"
    False,                 // r"\bfalse\b"
    Nil,                   // r"\bnil\b"
    And,                   // r"\band\b"
    Or,                    // r"\bor\b"
    Not,                   // r"\bnot\b"
    Equals,                // r"={1,2}|>=|<=|~="
    DoubleEquals,          // r"={1,2}|>=|<=|~="
    Dot,                   // r"\.{1}
    Colon,                 // r":"
    Comma,                 // r","
    LeftBracket,           // r"\["
    RightBracket,          // r"\]"
    LeftParenthesis,       // r"\("
    RightParenthesis,      // r"\)"
    LeftBrace,             // r"{"
    RightBrace,            // r"}"
    LeftShift,             // r"<<"
    RightShift,            // r">>"
    BitwiseAnd,            // r"&"
    BitwiseOr,             // r"\|"
    BitwiseNeg,            // r"~"
    Varargs,               // r"\.{3}"
    Semicolon,             // r";"
    Plus,                  // r"\+"
    Minus,                 // r"-"
    Multiply,              // r"\*"
    Divide,                // r"\/"
    Power,                 // r"\^"
    Modulo,                // r"%"
    Concat,                // r"\.{2}"
    LessThan,              // r"<"
    LessEq,                // r"={1,2}|>=|<=|~="
    GreaterThan,           // r">"
    GreaterEq,             // r"={1,2}|>=|<=|~="
    NotEq,                 // r"={1,2}|>=|<=|~="
    Length,                // r"#"
    Comment,               // r"--(?:(?:\[(=*)\[(?:\w|\s|\d)*\]\1\])|.*)"
    EndOfFile,
    Error(String),         // r"\b"
    Identifier(String),    // r"^[a-zA-Z_]\w*\b"
    StringLiteral(String), // r"(?:\[(=*)\[((?:.|\s)*)\]\3\])|(?:("|')(.*)\1)"
    NumberLiteral(f64)     // r"\d(\d|\.|((e|E)(\+|-)?)|\w)*"
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
