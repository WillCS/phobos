use std::fmt::Display;
use std::fmt::Formatter;

use crate::tokenisation::token::Token;

#[derive(Debug)]
pub struct TokenisationError {
    pub partial_token: Token,
    pub error_type:    TokenisationErrorType
}

#[derive(Debug)]
pub enum TokenisationErrorType {
    MalformedNumber,
    UnfinishedString,
    UnfinishedLongString,
    UnfinishedLongComment//,
    // Unimplemented
}

impl Display for TokenisationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self.error_type {
            TokenisationErrorType::MalformedNumber  => {
                write!(f, "{}: malformed number near {}",
                    self.partial_token.location.line,
                    self.partial_token
                )
            },
            TokenisationErrorType::UnfinishedString => {
                write!(f, "{}: unfinished string near {}",
                    self.partial_token.location.line,
                    self.partial_token
                )
            },
            TokenisationErrorType::UnfinishedLongString => {
                write!(f, "{}: unfinished long string (starting at line {}) near {}",
                    self.partial_token.location.line,
                    self.partial_token.location.line,
                    self.partial_token
                )
            },
            t @ _ => write!(f, "{}: {:?}", self.partial_token.location.line, t)
        }
    }
}