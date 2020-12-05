use std::fmt::Formatter;
use std::fmt::Display;
use std::str::{Chars, Lines};
use std::iter::{FromIterator, Peekable};
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
            TokenType::StringLiteral(name)   => write!(f, "String({}) at {}", name, self.location),
            TokenType::NumberLiteral(number) => write!(f, "Number({}) at {}", number, self.location),
            token @ _                          => write!(f, "{:?} at {} ", token, self.location)
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    End,                     // r"\bend\b"
    Do,                      // r"\bdo\b"
    While,                   // r"\bwhile\b"
    Repeat,                  // r"\brepeat\b"
    Until,                   // r"\buntil\b"
    If,                      // r"\bif\b"
    In,                      // r"\bin\b"
    Then,                    // r"\bthen\b"
    Elseif,                  // r"\belseif\b"
    Else,                    // r"\belse\b"
    For,                     // r"\bfor\b"
    Function,                // r"\bfunction\b"
    Local,                   // r"\blocal\b"
    Return,                  // r"\breturn\b"
    Break,                   // r"\bbreak\b"
    True,                    // r"\btrue\b"
    False,                   // r"\bfalse\b"
    Nil,                     // r"\bnil\b"
    And,                     // r"\band\b"
    Or,                      // r"\bor\b"
    Not,                     // r"\bnot\b"
    Equals,                  // r"={1,2}|>=|<=|~="
    DoubleEquals,            // r"={1,2}|>=|<=|~="
    Dot,                     // r"\.{1,3}
    Colon,                   // r":"
    Comma,                   // r","
    LeftBracket,             // r"\["
    RightBracket,            // r"\]"
    LeftParenthesis,         // r"\("
    RightParenthesis,        // r"\)"
    LeftBrace,               // r"{"
    RightBrace,              // r"}"
    Varargs,                 // r"\.{1,3}"
    Semicolon,               // r";"
    Plus,                    // r"\+"
    Minus,                   // r"-"
    Multiply,                // r"\*"
    Divide,                  // r"\/"
    Power,                   // r"\^"
    Modulo,                  // r"%"
    Concat,                  // r"\.{1,3}"
    LessThan,                // r"<"
    LessEq,                  // r"={1,2}|>=|<=|~="
    GreaterThan,             // r">"
    GreaterEq,               // r"={1,2}|>=|<=|~="
    NotEq,                   // r"={1,2}|>=|<=|~="
    Length,                  // r"#"
    Comment,                 // r"--(?:(?:\[(=*)\[(?:\w|\s|\d)*\]\1\])|.*)"
    Error,                   // r"\b"
    Identifier(String),      // r"^[a-zA-Z_]\w*\b"
    StringLiteral(String), // r"(?:\[(=*)\[((?:.|\s)*)\]\3\])|(?:("|')(.*)\1)"
    NumberLiteral(f64)     // r"\d(\d|\.|((e|E)(\+|-)?)|\w)*"
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

        return VecDeque::from_iter(token_deque.into_iter().map(|token| match token {
            Token { token_type: TokenType::Identifier(_), location: _ } => maybe_convert_to_keyword(token),
            _ => token
        }));
    }

    fn match_start_char(&mut self) -> Option<Token> {
        self.trim_whitespace();

        self.get_next_char()
            .as_ref()
            .map(|next_char| {
                self.constructing_token_location = Location { ..self.location };
                self.constructing_token.push(*next_char);
                
                let token = match next_char {
                    '_' | 'A'..='Z' | 'a'..='z' => self.parse_identifier(), // maybe an identifier, maybe a keyword
                    '0'..='9'                   => self.parse_number(),  // definitely a number
                    '\'' | '"'                  => self.parse_single_line_string(),  // definitely a string
                    _                           => self.get_error_token(),  // maybe a comment, maybe a symbol, maybe a string
                };
                
                self.trim_whitespace();

                return token;
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

    fn trim_whitespace(&mut self) {
        loop {
            let should_break = self.peek_next_char().map(|next_char| {
                let should_break = next_char.is_whitespace();
                if should_break {
                    self.get_next_char();
                }

                return should_break
            });

            if should_break.is_some() && !should_break.unwrap() {
                break;
            } else if should_break.is_none() {
                break;
            }
        }
    }

    fn parse_identifier(&mut self) -> Token {
        loop {
            let next_char = self.peek_next_char();

            let parsed_token = next_char.map(|c| match c {
                '_'|'A'..='Z' | 'a'..='z'|'0'..='9' => {
                    self.constructing_token.push(c);
                    false
                }
                _ => true
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

    fn parse_number(&mut self) -> Token {
        self.get_error_token()
    }
    
    fn parse_single_line_string(&mut self) -> Token {
        let opening_char = self.constructing_token.last().unwrap().clone();

        loop {
            let next_char = self.peek_next_char();

            let parsed_token = next_char.and_then(|c| match c {
                '\n' =>  None,
                _    => {
                    self.constructing_token.push(c);
                    self.get_next_char();
                    Some(c == opening_char)
                }
            });

            if parsed_token.is_some() {
                if parsed_token.unwrap() {
                    let contents = self.constructing_token.iter().collect::<String>().clone();
                    let len = contents.len() - 1;

                    return Token {
                        token_type: TokenType::StringLiteral(contents[1..len].to_string()),
                        location: Location { ..self.constructing_token_location }
                    }
                } else {
                    self.get_next_char();
                }
            } else {
                return self.get_error_token();
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

fn maybe_convert_to_keyword(token: Token) -> Token {
    Token {
        token_type: match token.token_type {
            TokenType::Identifier(name) => match name.as_str() {
                "end"      => TokenType::End,
                "do"       => TokenType::Do,
                "while"    => TokenType::While,
                "repeat"   => TokenType::Repeat,
                "until"    => TokenType::Until,
                "if"       => TokenType::If,
                "in"       => TokenType::In,
                "then"     => TokenType::Then,
                "elseif"   => TokenType::Elseif,
                "else"     => TokenType::Else,
                "for"      => TokenType::For,
                "function" => TokenType::Function,
                "local"    => TokenType::Local,
                "return"   => TokenType::Return,
                "break"    => TokenType::Break,
                "true"     => TokenType::True,
                "false"    => TokenType::False,
                "nil"      => TokenType::Nil,
                "and"      => TokenType::And,
                "or"       => TokenType::Or,
                "not"      => TokenType::Not,
                _          => TokenType::Identifier(name)
            },
            _ => token.token_type
        },
        location: token.location
    }
}
