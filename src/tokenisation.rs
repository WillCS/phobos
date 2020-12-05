use std::fmt::Formatter;
use std::fmt::Display;
use std::str::{Chars, Lines};
use std::iter::Peekable;
use std::vec::Vec;
use std::collections::VecDeque;
use regex::Regex;

use lazy_static::lazy_static;

// use regex::Regex;

lazy_static!{
    static ref identifier_regex: Regex = Regex::new(r"^[a-zA-Z_]\w*\b").unwrap();
}

pub struct Token {
    token_type: TokenType,
    location:   Location
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.token_type {
            TokenType::Identifier(name)        => write!(f, "Identifier({}) at {}", name, self.location),
            TokenType::StringPrimitive(name)   => write!(f, "String({}) at {}", name, self.location),
            TokenType::NumberPrimitive(number) => write!(f, "Number({}) at {}", number, self.location),
            token @ _                          => write!(f, "{:?} ", token)
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
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
    Identifier(String),      // r"^[a-zA-Z_]\w*\b"
    StringPrimitive(String), //r"(?:\[(=*)\[((?:.|\s)*)\]\3\])|(?:("|')(.*)\1)"
    NumberPrimitive(f64)      //r"\d(\d|\.|((e|E)(\+|-)?)|\w)*"
}

pub struct Location {
    pub line: usize,
    pub col:  usize
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "line {}, col {}", self.line, self.col)
    }
}

pub struct Tokeniser<'t> {
    pub location:                Location,
    current_line:                Option<Peekable<Chars<'t>>>,
    line_stack:                  Peekable<Lines<'t>>,
    constructing_token:          Vec<char>,
    constructing_token_location: Location
}

impl<'t> Tokeniser<'t> {
    pub fn new(source: &str) -> Tokeniser {
        let mut lines = source.lines();

        Tokeniser {
            location:                    Location { line: 1, col: 0 },
            current_line:                lines.next().map(|line| { line.chars().peekable() }),
            line_stack:                  lines.peekable(),
            constructing_token:          Vec::new(),
            constructing_token_location: Location { line: 0, col: 0}
        }
    }

    pub fn tokenise(&mut self) -> VecDeque<Token> {
        let mut token_deque = VecDeque::<Token>::new();

        let mut has_next_char = self.has_next_char();
        while has_next_char {
            self.constructing_token.clear();

            self.match_start_char().map(|token| {
                token_deque.push_back(token)
            });

            has_next_char = self.has_next_char();
        }

        return token_deque
    }

    fn match_start_char(&mut self) -> Option<Token> {
        self.get_next_char()
            .as_ref()
            .map(|next_char| {
                self.constructing_token_location = Location { ..self.location };
                self.constructing_token.push(*next_char);
                
                match next_char {
                    '_'                   => self.parse_identifier(),
                    'A'..='Z' | 'a'..='z' => self.parse_keyword_or_identifier(), // maybe an identifier, maybe a keyword
                    '0'..='9'             => self.get_error_token(), // definitely a number
                    '\'' | '"'            => self.get_error_token(), // definitely a string
                    _                     => self.get_error_token(), // maybe a comment, maybe a symbol, maybe a string
                }
            })
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        if self.current_line.is_none() {
            return None
        }

        self.location.col = self.location.col + 1;
        
        self.current_line
            .as_mut()
            .and_then(|line| { line.next() })
            .or_else(|| {               
                self.location.line = self.location.line + 1;
                self.location.col  = 0;

                self.current_line = self.line_stack.next().map(|line| { 
                    line.chars().peekable()
                });
                
                Some('\n')
                // self.get_next_char()
            })
    }

    pub fn peek_next_char(&mut self) -> Option<char> {
        if self.current_line.is_none() {
            return None
        }

        let has_next_line = self.line_stack.peek().is_some();

        self.current_line
            .as_mut()
            .and_then(|line| { line.peek() })
            .map(|c| { c.clone() })
            .or_else(|| {
                if has_next_line {
                    Some('\n')
                } else {
                    None
                }
            })
    }

    pub fn has_next_char(&mut self) -> bool {
        self.current_line
            .as_mut()
            .map(|line| line.peek().is_some())
            .unwrap_or(false) || 
        self.line_stack.peek()
            .map(|line| !line.is_empty())
            .unwrap_or(false)
    }

    fn parse_identifier(&mut self) -> Token {
        loop {
            let next_char = self.peek_next_char();

            let parsed_token = next_char.map(|c| {
                match c {
                    '_'|'A'..='Z' | 'a'..='z'|'0'..='9' => {
                        self.constructing_token.push(c);
                        false
                    }
                    _ => true
                }
            }).unwrap_or(true);

            if parsed_token {
                let identifier = self.constructing_token.iter().collect::<String>().clone();
                return Token {
                    token_type: TokenType::Identifier(identifier),
                    location:   Location { ..self.constructing_token_location }
                }
            } else {
                self.get_next_char();
            }
        }
    }

    fn parse_keyword_or_identifier(&mut self) -> Token {
        let next_char = self.peek_next_char();

        next_char.map(|c| {
            match c {
                'h' if *self.constructing_token.last().unwrap() == 'w' => self.parse_while_keyword(),
                _ => self.parse_identifier()
            }

        }).unwrap()
    }

    fn parse_while_keyword(&mut self) -> Token {
        loop {
            let next_char = self.peek_next_char();

            let parsed_token = next_char.and_then(|c| {
                let last_char = *self.constructing_token.last().unwrap();
                match c {
                    'h' if last_char == 'w' => {
                        self.constructing_token.push(c);
                        Some(false)
                    },
                    'i' if last_char == 'h' => {
                        self.constructing_token.push(c);
                        Some(false)
                    },
                    'l' if last_char == 'i' => {
                        self.constructing_token.push(c);
                        Some(false)
                    },
                    'e' if last_char == 'l' => {
                        self.constructing_token.push(c);
                        Some(false)
                    },
                    _ if last_char == 'e' => Some(true),
                    _ => None
                }
            });
            
            if parsed_token.is_some() && parsed_token.unwrap() {
                return Token {
                    token_type: TokenType::While,
                    location:   Location { ..self.constructing_token_location }
                }
            } else if parsed_token.is_some() {
                self.get_next_char();
            } else {
                return self.parse_identifier();
            }
        }
    }

    fn get_error_token(&mut self) -> Token {
        Token {
            token_type: TokenType::Error,
            location:   Location { ..self.constructing_token_location }
        }
    }
}
