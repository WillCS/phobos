use std::vec::Vec;
use regex::Regex;

use lazy_static::lazy_static;

// use regex::Regex;

lazy_static!{
    static ref identifier_regex: Regex = Regex::new(r"^[a-zA-Z_]\w*\b").unwrap();
}

pub struct Token<'a> {
    token_type: TokenType<'a>,
    location:   Location
}

pub enum TokenType<'a> {
    End,                      // r"\bend\b"
    Do,                       // r"\bdo\b"
    While,                    // r"\bwhile\b"
    Repeat,                   // r"\brepeat\b"
    Until,                    // r"\buntil\b"
    If,                       // r"\bif\b"
    In,                       // r"\bin\b"
    Then,                     // r"\bthen\b"
    Elseif,                   // r"\belseif\b"
    Else,                     // r"\belse\b"
    For,                      // r"\bfor\b"
    Function,                 // r"\bfunction\b"
    Local,                    // r"\blocal\b"
    Return,                   // r"\breturn\b"
    Break,                    // r"\bbreak\b"
    True,                     // r"\btrue\b"
    False,                    // r"\bfalse\b"
    Nil,                      // r"\bnil\b"
    And,                      // r"\band\b"
    Or,                       // r"\bor\b"
    Not,                      // r"\bnot\b"
    Equals,                   // r"={1,2}|>=|<=|~="
    DoubleEquals,             // r"={1,2}|>=|<=|~="
    Dot,                      // r"\.{1,3}
    Colon,                    // r":"
    Comma,                    // r","
    LeftBracket,              // r"\["
    RightBracket,             // r"\]"
    LeftParenthesis,          // r"\("
    RightParenthesis,         // r"\)"
    LeftBrace,                // r"{"
    RightBrace,               // r"}"
    Varargs,                  // r"\.{1,3}"
    Semicolon,                // r";"
    Plus,                     // r"\+"
    Minus,                    // r"-"
    Multiply,                 // r"\*"
    Divide,                   // r"\/"
    Power,                    // r"\^"
    Modulo,                   // r"%"
    Concat,                   // r"\.{1,3}"
    LessThan,                 // r"<"
    LessEq,                   // r"={1,2}|>=|<=|~="
    GreaterThan,              // r">"
    GreaterEq,                // r"={1,2}|>=|<=|~="
    NotEq,                    // r"={1,2}|>=|<=|~="
    Length,                   // r"#"
    Comment,                  // r"--(?:(?:\[(=*)\[(?:\w|\s|\d)*\]\1\])|.*)"
    Error,                    // r"\b"
    Identifier(&'a str),      // r"^[a-zA-Z_]\w*\b"
    StringPrimitive(&'a str), //r"(?:\[(=*)\[((?:.|\s)*)\]\3\])|(?:("|')(.*)\1)"
    NumberPrimitive(f64)      //r"\d(\d|\.|((e|E)(\+|-)?)|\w)*"
}

pub struct Location {
    line: usize,
    col:  usize
}

struct TextInput<'a> {
    location: Location,
    current_line: &'a str,
    line_stack: Vec<&'a str>,
}

impl TextInput<'_> {
    pub fn pop_token(&mut self) -> Result<Token, &str> {
        self.pop_line_if_empty().and_then(|_| {
            Ok(self.read_token())
        })
    }

    fn read_token(&mut self) -> Token {
        self.trim_current_line();

        match self.current_line.chars().next() {
            None             => TokenType::Error,
            Some(first_char) => match first_char {
                '_'                   => Token {
                    token_type: match parse_identifier(self.current_line) {
                        None       => TokenType::Error,
                        Some(name) => TokenType::Identifier(name)
                    },
                    location: Location { ..self.location }
                }, // definitely an identifier
                'A'..='Z' | 'a'..='z' => None, // maybe an identifier, maybe a keyword
                '0'..='9'             => None, // definitely a number
                '\'' | '"'            => None, // definitely a string
                _                     => None, // maybe a comment, maybe a symbol, maybe a string
            }
        }

        Token {
            token_type: TokenType::Error,
            location:   Location { ..self.location }
        }
    }

    fn trim_current_line(&mut self) {
        let len_before_trim = self.current_line.chars().count();

        self.current_line = self.current_line.trim_start();

        let len_after_trim = self.current_line.chars().count();

        if len_before_trim != len_after_trim {
            self.location.col += len_before_trim - len_after_trim;
        }
    }

    fn pop_line(&mut self) -> Result<(), &str> {
        self.location.line = self.location.line + 1;

        match self.line_stack.pop() {
            None       => Err("reached end of file"),
            Some(line) => Ok(self.current_line = &[self.current_line, line].concat())
        }
    }

    fn pop_line_if_empty(&mut self) -> Result<(), &str> {
        if self.current_line.is_empty() {
            self.pop_line()
        } else {
            Ok(())
        }
    }
}

fn parse_identifier(line: &str) -> Option<&str> {
    identifier_regex.find(line).map(|mat| {
        mat.as_str()
    })
}
