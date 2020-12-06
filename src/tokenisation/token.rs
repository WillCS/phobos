use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location:   Location
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.token_type {
            TokenType::Identifier(name)      => write!(f, "{}", name),
            TokenType::StringLiteral(value)  => write!(f, "'{}'", value),
            TokenType::NumberLiteral(number) => write!(f, "'{}'", number),
            TokenType::Error(value)          => write!(f, "'{}'", value),
            TokenType::EndOfFile             => write!(f, "<eof>"),
            token @ _                        => write!(f, "{:?} at {} ", token, self.location)
        }
    }
}

#[derive(Debug)]
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
    Dot,                   // r"\.{1,3}
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
    Varargs,               // r"\.{1,3}"
    Semicolon,             // r";"
    Plus,                  // r"\+"
    Minus,                 // r"-"
    Multiply,              // r"\*"
    Divide,                // r"\/"
    Power,                 // r"\^"
    Modulo,                // r"%"
    Concat,                // r"\.{1,3}"
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
