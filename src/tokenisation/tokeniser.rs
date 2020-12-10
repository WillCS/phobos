use std::marker::PhantomData;
use std::fmt::Display;
use std::str::Lines;
use std::iter::Peekable;
use std::vec::Vec;
use std::collections::{VecDeque, HashMap};

use regex::Regex;

use crate::tokenisation::token::{Token, Location};
use crate::tokenisation::error::TokenisationError;

pub enum LexemeMatcher {
    WholeMatcher(Regex),
    StartMatcher(Regex)
}

pub enum LexemeBuilder<'t, T, U> where T: Display, T: Clone {
    StaticBuilder(T),
    DynamicBuilder(&'t dyn Fn(String, Location) -> Result<Token<T>, TokenisationError<T, U>>),
    ComplicatedBuilder(&'t dyn Fn(&mut TokeniserState<T, U>, Location) -> Result<Token<T>, TokenisationError<T, U>>)
}

pub struct LexemeTokeniser<'t, T, U> where T: Display, T: Clone {
    pub matcher: LexemeMatcher,
    pub builder: LexemeBuilder<'t, T, U>
}

pub struct TokeniserState<'t, T, U> where T: Display, T: Clone {
    pub location:                Location,
    pub line_buffer:             Option<String>,
    line_stack:                  Peekable<Lines<'t>>,
    phantom_token_type:          PhantomData<T>,
    phantom_error_type:          PhantomData<U>
}

impl<T, U> TokeniserState<'_, T, U> where T: Display, T: Clone {
    pub fn get_token(&mut self,
        tokeniser: &Tokeniser<T, U>
    ) -> Result<Token<T>, TokenisationError<T, U>> {
        self.trim_whitespace();

        while self.is_end_of_line() {
            self.pop_line();
            self.trim_whitespace();

            if self.is_end_of_file() {
                return Ok((tokeniser.eof_handler)(self.location));
            }
        }

        let start_location = self.location;
        
        let current_line = self.line_buffer.as_ref().unwrap();

        for lexeme in &tokeniser.lexemes {
            let matches = match &lexeme.matcher {
                LexemeMatcher::WholeMatcher(matcher)       => matcher.is_match(current_line),
                LexemeMatcher::StartMatcher(start_matcher) => start_matcher.is_match(current_line)
            };

            if matches {
                match &lexeme.matcher {
                    LexemeMatcher::WholeMatcher(matcher)       => println!("{}", matcher),
                    LexemeMatcher::StartMatcher(start_matcher) => println!("{}", start_matcher)
                };
                return self.match_lexeme(&lexeme, start_location);
            }
        }

        let first_char = current_line.chars().next().unwrap();

        for (start_char, handler) in &tokeniser.error_handlers {
            if first_char == *start_char {
                return Err(handler(start_location, String::from(current_line.clone())));
            }
        }

        return Ok((tokeniser.eof_handler)(start_location));
    }

    fn match_lexeme(&mut self,
        lexeme:         &LexemeTokeniser<T, U>,
        start_location: Location
    ) -> Result<Token<T>, TokenisationError<T, U>> {
        let current_line = self.line_buffer.as_ref().unwrap().clone();

        match &lexeme.matcher {
            LexemeMatcher::WholeMatcher(matcher) => {
                let lexeme_match = matcher.find(&current_line[..]).unwrap().as_str().to_string();
                self.build_token(&lexeme.builder, lexeme_match, start_location)
            },
            LexemeMatcher::StartMatcher(start_matcher) => {
                let start_match = start_matcher.find(&current_line[..]).unwrap().as_str().to_string();

                self.build_token(&lexeme.builder, start_match, start_location)
            }
        }
    }

    fn build_token(&mut self,
        builder:        &LexemeBuilder<T, U>,
        value:          String,
        start_location: Location
    ) -> Result<Token<T>, TokenisationError<T, U>> {
        match builder {
            LexemeBuilder::StaticBuilder(token_type) => {
                let chars_to_consume = value.len();
                self.consume_chars(chars_to_consume);
                Ok(Token {
                    token_type: (*token_type).clone(),
                    location:   start_location
                })
            },
            LexemeBuilder::DynamicBuilder(builder_fn) => {
                let chars_to_consume = value.len();
                self.consume_chars(chars_to_consume);

                builder_fn(value, start_location)
            },
            LexemeBuilder::ComplicatedBuilder(handler_fn) => {
                handler_fn(self, start_location)
            }
        }
    }

    pub fn is_end_of_line(&mut self) -> bool {
        self.line_buffer
            .as_mut()
            .map(|line| {
                line.is_empty() || line.len() == 0
            })
            .unwrap_or(true)
    }

    pub fn has_next_line(&mut self) -> bool {
        self.line_stack.peek().is_some()
    }

    pub fn is_end_of_file(&mut self) -> bool {
        self.is_end_of_line() && self.line_stack.peek().is_none()
    }

    pub fn trim_whitespace(&mut self) {
        if self.line_buffer.is_some() {
            let line = self.line_buffer.as_ref().unwrap().clone();

            let trimmed_line = line.trim_start();
            let trimmed_chars = line.len() - trimmed_line.len();

            if trimmed_chars != 0 {
                self.line_buffer.replace(String::from(trimmed_line));
                self.location.col = self.location.col + trimmed_chars;
            }
        }
    }

    pub fn pop_line(&mut self) {
        if self.line_stack.peek().is_some() {
            let mut next_line = String::from(self.line_stack.next().unwrap());
            next_line.push('\n');

            if self.line_buffer.is_some() {
                self.line_buffer.as_mut().unwrap().push_str(&next_line[..]);
            } else {
                self.line_buffer.replace(next_line);
            }
            self.location.line = self.location.line + 1;
            self.location.col  = 1;
        }
    }

    pub fn consume_chars(&mut self, num_chars: usize) {
        if self.line_buffer.is_some() {
            let line = self.line_buffer.as_ref().unwrap();
            self.location.col = self.location.col + num_chars;
            self.line_buffer.replace(line[num_chars..].to_string());
        }
    }
}

pub struct Tokeniser<'t, T, U> where T: Display, T: Clone {
    lexemes:        Vec<LexemeTokeniser<'t, T, U>>,
    error_handlers: HashMap<char, &'t dyn Fn(Location, String) -> TokenisationError<T, U>>,
    state:          Option<TokeniserState<'t, T, U>>,
    eof_handler:    &'t dyn Fn(Location) -> Token<T>
}

impl<'t, T, U> Tokeniser<'t, T, U> where T: Display, T: Clone {
    pub fn new(
        lexemes:        Vec<LexemeTokeniser<'t, T, U>>,
        error_handlers: HashMap<char, &'t dyn Fn(Location, String) -> TokenisationError<T, U>>,
        eof_handler:    &'t dyn Fn(Location) -> Token<T>
    ) -> Tokeniser<'t, T, U> {
        Tokeniser {
            lexemes:        lexemes,
            error_handlers: error_handlers,
            state:          None,
            eof_handler:    eof_handler
        }
    }

    pub fn tokenise(&mut self, src: String) -> Result<VecDeque<Token<T>>, TokenisationError<T, U>> {
        let mut token_deque: VecDeque<Token<T>> = VecDeque::new();
        let mut lines = src.lines();
        
        let mut state = TokeniserState::<T, U> {
            location:           Location { line: 1, col: 1},
            line_buffer:        lines.next().map(|line| line.to_string()),
            line_stack:         lines.peekable(),
            phantom_token_type: PhantomData,
            phantom_error_type: PhantomData
        };

        while !state.is_end_of_file() {
            token_deque.push_back(state.get_token(self)?);
        }

        Ok(token_deque)
    }
}