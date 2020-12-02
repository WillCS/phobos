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
    Identifier(String),      // r"^[a-zA-Z_]\w*\b"
    StringPrimitive(&'a str), //r"(?:\[(=*)\[((?:.|\s)*)\]\3\])|(?:("|')(.*)\1)"
    NumberPrimitive(f64)      //r"\d(\d|\.|((e|E)(\+|-)?)|\w)*"
}

pub struct Location {
    pub line: usize,
    pub col:  usize
}

pub struct Tokeniser<'t> {
    pub location:                Location,
    current_line:                Option<Peekable<Chars<'t>>>,
    line_stack:                  Peekable<Lines<'t>>,
    constructing_token:          Vec<char>,
    constructing_token_location: Location
}

impl Tokeniser<'_> {
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

        while self.has_next_char() {
            let optional_next_char = self.get_next_char();
            let matched_token = optional_next_char
                .map(|next_char| {
                    self.constructing_token_location = Location { ..self.location };
                    self.constructing_token.push(next_char);
                    
                    match next_char {
                        '_'                   => self.parse_identifier(),
                        'A'..='Z' | 'a'..='z' => self.get_error_token(), // maybe an identifier, maybe a keyword
                        '0'..='9'             => self.get_error_token(), // definitely a number
                        '\'' | '"'            => self.get_error_token(), // definitely a string
                        _                     => self.get_error_token(), // maybe a comment, maybe a symbol, maybe a string
                    }
                }).map(|token| {
                    token_deque.push_back(token);
                });
        }

        return token_deque
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
            }).unwrap();

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

        // self.get_next_char().and_then(|next_char| {

        // }).unwrap_or(self.get_error_token())
    }

    fn get_error_token(&mut self) -> Token {
        Token {
            token_type: TokenType::Error,
            location:   Location { ..self.constructing_token_location }
        }
    }
}
