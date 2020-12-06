use std::str::{Chars, Lines};
use std::iter::Peekable;
use std::vec::Vec;
use std::collections::VecDeque;

use crate::tokenisation::token::{Token, TokenType, Location};
use crate::tokenisation::error::{TokenisationError, TokenisationErrorType};

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
                    '-'                         => self.parse_comment_or_minus(),
                    '['                         => self.parse_long_string_or_bracket(),
                    _                           => self.parse_symbol(),  // maybe a comment, maybe a symbol, maybe a string
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

        if still_parsing && next_char.is_none() {
            errored = true;
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
                'A'..='Z' | 'a'..='z' | '0'..='9' | '.' | '_' => {},
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

    fn parse_comment_or_minus(&mut self) -> Result<Token, TokenisationError> {
        let mut next_char = self.peek_next_char();

        match next_char {
            Some(c @ '-') => {
                self.constructing_token.push(c);
                self.get_next_char();

                match self.peek_next_char() {
                    Some('[') => {
                        self.constructing_token.push(c);
                        self.get_next_char();
                        
                        match self.parse_multiline() {
                            Some(_) => {
                                Ok(Token {
                                    token_type: TokenType::Comment,
                                    location:   self.constructing_token_location
                                })
                            },
                            None => Err(self.get_tokenisation_error(TokenisationErrorType::UnfinishedLongComment))
                        }
                    },
                    _ => {
                        next_char = self.peek_next_char();
                        while next_char.is_some() && next_char.unwrap() != '\n' {
                            self.constructing_token.push(c);
                            self.get_next_char();
                            next_char = self.peek_next_char();
                        };

                        Ok(Token {
                            token_type: TokenType::Comment,
                            location:   self.constructing_token_location
                        })

                    }
                }
            },
            _ => {
                Ok(Token {
                    token_type: TokenType::Minus,
                    location:   self.constructing_token_location
                })
            }
        }
    }

    fn parse_long_string_or_bracket(&mut self) -> Result<Token, TokenisationError> {
        let next_char  = self.peek_next_char();

        match next_char {
            Some('=') | Some('[') => match self.parse_multiline() {
                Some((value, depth)) => {
                    let string_len      = value.len();
                    let to_cut_out      = depth + 2;
                    let string_contents = &value[to_cut_out..(string_len - to_cut_out)];

                    Ok(Token {
                        token_type: TokenType::StringLiteral(String::from(string_contents)),
                        location:   self.constructing_token_location
                    })
                },
                None => Err(self.get_tokenisation_error(TokenisationErrorType::UnfinishedLongString))
            },
            _ => Ok(Token {
                token_type: TokenType::LeftBracket,
                location:   self.constructing_token_location
            })
        }
    }

    fn parse_symbol(&mut self) -> Result<Token, TokenisationError> {
        let first_char  = self.constructing_token.last().unwrap().clone();

        let token_type = match first_char {
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '[' => TokenType::RightBracket,
            ']' => TokenType::RightBracket,
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ';' => TokenType::Semicolon,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Multiply,
            '^' => TokenType::Power,
            '/' => TokenType::Divide,
            '%' => TokenType::Modulo,
            '#' => TokenType::Length,
            '&' => TokenType::BitwiseAnd,
            '|' => TokenType::BitwiseOr,
            '<' => {
                 match self.peek_next_char() {
                     Some(unwrapped_char @ '=') => {
                        self.constructing_token.push(unwrapped_char);
                        TokenType::LessEq
                     },
                     Some(unwrapped_char @ '>') => {
                        self.constructing_token.push(unwrapped_char);
                        TokenType::LeftShift
                     },
                     _ => {
                         TokenType::LessThan
                     }
                }
            },
            '>' => {
                match self.peek_next_char() {
                    Some(unwrapped_char @ '=') => {
                       self.constructing_token.push(unwrapped_char);
                       TokenType::GreaterEq
                    },
                    Some(unwrapped_char @ '>') => {
                       self.constructing_token.push(unwrapped_char);
                       TokenType::RightShift
                    },
                    _ => {
                        TokenType::GreaterThan
                    }
                }
            },
            '=' => {
                match self.peek_next_char() {
                    Some(unwrapped_char @ '=') => {
                       self.constructing_token.push(unwrapped_char);
                       TokenType::DoubleEquals
                    },
                    _ => {
                        TokenType::Equals
                    }
                }
            },
            '~' => {
                match self.peek_next_char() {
                    Some(unwrapped_char @ '=') => {
                       self.constructing_token.push(unwrapped_char);
                       TokenType::NotEq
                    },
                    _ => {
                        TokenType::BitwiseNeg
                    }
                }
            },
            '.' => {
                match self.peek_next_char() {
                    Some(unwrapped_char @ '.') => {
                       self.constructing_token.push(unwrapped_char);
                       self.get_next_char();

                       match self.peek_next_char() {
                            Some(unwrapped_char @ '.') => {
                                self.constructing_token.push(unwrapped_char);
                                self.get_next_char();
                                TokenType::Varargs
                            },
                            _ => {
                                TokenType::Concat
                            }
                        }
                    },
                    _ => {
                        TokenType::Dot
                    }
               }
            },
            _ => TokenType::Error(self.collect_token())
        };

        Ok(Token {
            token_type: token_type,
            location:   self.constructing_token_location
        })
    }

    fn parse_multiline(&mut self) -> Option<(String, usize)> {
        let mut depth = 0;
        let mut next_char = self.peek_next_char();

        while next_char.is_some() && next_char.unwrap() == '=' {
            let unwrapped_char = next_char.unwrap();
            self.constructing_token.push(unwrapped_char);
            depth += 1;

            self.get_next_char();
            next_char = self.peek_next_char();
        }

        if next_char.is_some() && next_char.unwrap() == '[' {
            let unwrapped_char = next_char.unwrap();
            self.constructing_token.push(unwrapped_char);

            self.get_next_char();
            next_char = self.peek_next_char();
        } else {
            return None;
        }

        loop {
            match next_char {
                Some(']') => {
                    let unwrapped_char = next_char.unwrap();
                    self.constructing_token.push(unwrapped_char);
        
                    self.get_next_char();
                    next_char = self.peek_next_char();

                    let mut comp_depth = 0;

                    while next_char.is_some() && next_char.unwrap() == '=' {
                        let unwrapped_char = next_char.unwrap();
                        self.constructing_token.push(unwrapped_char);
                        comp_depth += 1;

                        self.get_next_char();
                        next_char = self.peek_next_char();
                    }

                    if next_char.is_some() && next_char.unwrap() == ']' {
                        let unwrapped_char = next_char.unwrap();
                        self.constructing_token.push(unwrapped_char);
                        self.get_next_char();
                        
                        if comp_depth == depth {
                            break
                        } else {
                            next_char = self.peek_next_char();
                        }
                    }
                },
                Some(_) => {
                    let unwrapped_char = next_char.unwrap();
                    self.constructing_token.push(unwrapped_char);
        
                    self.get_next_char();
                    next_char = self.peek_next_char();
                },
                None => {
                    return None;
                }
            }
        }

        return Some((self.collect_token(), depth))
    }

    fn collect_token(&mut self) -> String {
        self.constructing_token.iter().collect::<String>().clone()
    }

    fn get_tokenisation_error(&mut self, error_type: TokenisationErrorType) -> TokenisationError {
        TokenisationError {
            partial_token: self.get_error_token(),
            error_type:    error_type
        }
    }

    fn get_error_token(&mut self) -> Token {
        Token {
            token_type: TokenType::Error(self.collect_token()),
            location:   self.constructing_token_location
        }
    }

    fn get_eof_token(&self) -> Token {
        Token {
            token_type: TokenType::EndOfFile,
            location:   self.constructing_token_location
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
