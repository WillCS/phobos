use std::fmt::Display;

use crate::tokenisation::token::Token;

#[derive(Debug)]
pub struct TokenisationError<T, U> where T: Display {
    pub partial_token: Token<T>,
    pub error_type:    U
}

#[derive(Debug)]
pub enum TokenisationErrorType {
    MalformedNumber,
    UnfinishedString,
    UnfinishedLongString,
    UnfinishedLongComment,
    SyntaxError,
    Unimplemented
}

// impl<T, U> Display for TokenisationError<T, U> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         match &self.error_type {
//             TokenisationErrorType::MalformedNumber  => {
//                 write!(f, "{}: malformed number near {}",
//                     self.partial_token.location.line,
//                     self.partial_token
//                 )
//             },
//             TokenisationErrorType::UnfinishedString => {
//                 write!(f, "{}: unfinished string near {}",
//                     self.partial_token.location.line,
//                     self.partial_token
//                 )
//             },
//             TokenisationErrorType::UnfinishedLongString => {
//                 write!(f, "{}: unfinished long string (starting at line {}) near {}",
//                     self.partial_token.location.line,
//                     self.partial_token.location.line,
//                     self.partial_token
//                 )
//             },
//             t @ _ => write!(f, "{}: {:?}", self.partial_token.location.line, t)
//         }
//     }
// }