use std::fmt::Formatter;
use std::fmt::Display;
use std::str::{Chars, Lines};
use std::iter::{FromIterator, Peekable};
use std::vec::Vec;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct TokenisationError {
    pub partial_token: String,
    pub location:      Location,
    pub error_type:    TokenisationErrorType
}

#[derive(Debug)]
pub enum TokenisationErrorType {
    MalformedNumber,
    UnfinishedString,
    UnfinishedLongString,
    Unimplemented
}

pub struct Token {
    token_type: TokenType,
    location:   Location
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.token_type {
            TokenType::Identifier(name)      => write!(f, "Identifier({}) at {}", name, self.location),
            TokenType::StringLiteral(name)   => write!(f, "String({}) at {}", name, self.location),
            TokenType::NumberLiteral(number) => write!(f, "Number({}) at {}", number, self.location),
            token @ _                        => write!(f, "{:?} at {} ", token, self.location)
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
    EndOfFile,
    Identifier(String),      // r"^[a-zA-Z_]\w*\b"
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

    pub fn tokenise(&mut self) -> Result<VecDeque<Token>, TokenisationError> {
        let mut token_deque = VecDeque::<Token>::new();

        let mut has_next_char = self.has_next_char();
        while has_next_char {
            self.constructing_token.clear();

            self.match_start_char().map(|token| {
                println!("{}", token);
                token_deque.push_back(token)
            })?;

            has_next_char = self.has_next_char();
        }

        Ok(token_deque)
    }

    fn match_start_char(&mut self) -> Result<Token, TokenisationError> {
        self.trim_whitespace();

        self.get_next_char()
            .as_ref()
            .map(|next_char| {
                self.constructing_token_location = self.location;
                self.constructing_token.push(*next_char);
                
                let token = match next_char {
                    '_' | 'A'..='Z' | 'a'..='z' => self.parse_identifier(), // maybe an identifier, maybe a keyword
                    '0'..='9'                   => self.parse_number(),  // definitely a number
                    '\'' | '"'                  => self.parse_single_line_string(),  // definitely a string
                    _                           => Err(self.get_tokenisation_error(TokenisationErrorType::Unimplemented)),  // maybe a comment, maybe a symbol, maybe a string
                };
                
                self.trim_whitespace();

                return token;
            }).unwrap_or(Ok(self.get_eof_token()))
    }

    fn get_next_char(&mut self) -> Option<char> {
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
            })
    }

    fn peek_next_char(&mut self) -> Option<char> {
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

    fn has_next_char(&mut self) -> bool {
        self.current_line
            .as_mut()
            .map(|line| line.peek().is_some())
            .unwrap_or(false) || 
        self.line_stack.peek()
            .map(|line| !line.is_empty())
            .unwrap_or(false)
    }

    fn trim_whitespace(&mut self) {
        let mut next_char = self.peek_next_char();

        while next_char.is_some() && next_char.unwrap().is_whitespace() {
            self.get_next_char();
            next_char = self.peek_next_char();
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, TokenisationError> {
        let mut next_char = self.peek_next_char();
        let mut still_parsing = true;

        while next_char.is_some() && still_parsing {
            let unwrapped_char = next_char.unwrap();
            match unwrapped_char {
                '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    self.constructing_token.push(unwrapped_char);
                    self.get_next_char();
                    next_char = self.peek_next_char();
                },
                _ => still_parsing = false
            };
        }

        let identifier = self.collect_token();

        Ok(self.create_token_from_name(identifier))
    }
    
    fn parse_single_line_string(&mut self) -> Result<Token, TokenisationError> {
        let opening_char  = self.constructing_token.last().unwrap().clone();
        let mut next_char = self.peek_next_char();
        let mut still_parsing = true;
        let mut errored       = false;

        while next_char.is_some() && still_parsing {
            let unwrapped_char = next_char.unwrap();
            match unwrapped_char {
                '\n' =>  {
                    still_parsing = false;
                    errored       = true;
                },
                _    => {
                    self.constructing_token.push(unwrapped_char);
                    self.get_next_char();
                    next_char = self.peek_next_char();
                    still_parsing = unwrapped_char != opening_char;
                }
            };
        }

        if errored {
            Err(self.get_tokenisation_error(TokenisationErrorType::UnfinishedString))
        } else {
            let string_contents = self.collect_token();
            let string_len      = string_contents.len() - 1;

            Ok(Token {
                token_type: TokenType::StringLiteral(string_contents[1..string_len].to_string()),
                location:   self.constructing_token_location
            })
        }
    }

    fn parse_number(&mut self) -> Result<Token, TokenisationError> {
        let mut next_char = self.peek_next_char();
        let mut still_parsing = true;

        let mut prev_char_e   = false;

        while next_char.is_some() && still_parsing {
            let unwrapped_char = next_char.unwrap();
            match unwrapped_char {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '.' => {},
                '+' | '-' if prev_char_e                => {},
                _ => { still_parsing = false; }
            }

            if still_parsing {
                self.constructing_token.push(unwrapped_char);
                self.get_next_char();
                next_char = self.peek_next_char();
            }

            prev_char_e = unwrapped_char == 'e';
        }

        self.collect_token()
            .parse::<f64>()
            .and_then(|num| {
                Ok(Token {
                    token_type: TokenType::NumberLiteral(num),
                    location:   self.constructing_token_location
                })
            })
            .or_else(|_| {
                Err(self.get_tokenisation_error(TokenisationErrorType::MalformedNumber))
            })
    }

    fn collect_token(&mut self) -> String {
        self.constructing_token.iter().collect::<String>().clone()
    }

    fn get_tokenisation_error(&mut self, error_type: TokenisationErrorType) -> TokenisationError {
        TokenisationError {
            partial_token: self.collect_token(),
            location:      self.location,
            error_type:    error_type
        }
    }

    fn get_eof_token(&mut self) -> Token {
        Token {
            token_type: TokenType::EndOfFile,
            location:   self.location
        }
    }

    fn create_token_from_name(&mut self, name: String) -> Token {
        Token {
            token_type: match name.as_str() {
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
            location: self.constructing_token_location
        }
    }
}
