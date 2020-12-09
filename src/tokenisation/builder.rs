use std::fmt::Display;
use std::vec::Vec;
use regex::Regex;


use crate::tokenisation::tokeniser::{Tokeniser, LexemeTokeniser, LexemeBuilder, LexemeMatcher, TokeniserState};
use crate::tokenisation::token::{Token, Location};
use crate::tokenisation::error::TokenisationError;

pub struct TokeniserBuilder<'t, T, U> where T: Display, T: Clone {
    lexemes:     Vec<LexemeTokeniser<'t, T, U>>,
    eof_handler: Option<&'t dyn Fn(Location) -> Token<T>>
}

impl<'t, T, U> TokeniserBuilder<'t, T, U> where T: Display, T: Clone {
    pub fn new<'u, V, W>() -> TokeniserBuilder<'u, V, W> where V: Display, V: Clone {
        TokeniserBuilder::<'u, V, W> {
            lexemes:     Vec::new(),
            eof_handler: None
        }
    }

    pub fn with_static_token(mut self,
        token_matcher: Regex,
        token_type:    T
    ) -> TokeniserBuilder<'t, T, U> {
        self.lexemes.push(LexemeTokeniser {
            matcher: LexemeMatcher::WholeMatcher(token_matcher),
            builder: LexemeBuilder::StaticBuilder(token_type)
        });
        return self;
    }
    
    pub fn with_dynamic_token(mut self,
        token_matcher: Regex,
        token_parser:  &'t dyn Fn(String, Location) -> Result<Token<T>, TokenisationError<T, U>>
    ) -> TokeniserBuilder<'t, T, U> {
        self.lexemes.push(LexemeTokeniser {
            matcher: LexemeMatcher::WholeMatcher(token_matcher),
            builder: LexemeBuilder::DynamicBuilder(token_parser)
        });
        return self;
    }

    pub fn with_complicated_token(mut self,
        token_start_matcher: Regex,
        token_parser:        &'t dyn Fn(&mut TokeniserState<T, U>, Location) -> Result<Token<T>, TokenisationError<T, U>>
    ) -> TokeniserBuilder<'t, T, U> {
        self.lexemes.push(LexemeTokeniser {
            matcher: LexemeMatcher::StartMatcher(token_start_matcher),
            builder: LexemeBuilder::ComplicatedBuilder(token_parser)
        });
        return self;
    }

    pub fn with_eof_handler(mut self,
        handler: &'t dyn Fn(Location) -> Token<T>
    ) -> TokeniserBuilder<'t, T, U> {
        self.eof_handler = Some(handler);
        return self;
    }
    
    pub fn build(self) -> Option<Tokeniser<'t, T, U>> {
        if self.eof_handler.is_none() {
            return None
        }
        
        Some(Tokeniser::new(self.lexemes, self.eof_handler.unwrap()))
    }
}
