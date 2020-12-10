use std::fs;

use regex::Regex;
use lazy_static::lazy_static;

mod tokenisation;

use tokenisation::{TokeniserState, TokeniserBuilder, Tokeniser, TokenType, Token, TokenisationError, Location, TokenisationErrorType};

lazy_static!{
    pub static ref MULTILINE_START_REGEX:  Regex = Regex::new(r"^\[=*\[").unwrap();
    pub static ref MULTILINE_FINISH_REGEX: Regex = Regex::new(r"\]=*\]").unwrap();
}

fn main() {
    let contents = fs::read_to_string("input/test.lua")
        .expect("Something bad happened");

    // let mut tokeniser = tokenisation::Tokeniser::new(&contents);

    // match tokeniser.tokenise() {
    //     Ok(tokens) => {
    //         for token in tokens {
    //             println!("{:?}", token)
    //         }
    //     },
    //     Err(error) => {
    //         println!("{}", error)::<tokenisation::TokenType>
    //     }
    // }

    let tokeniser: Option<Tokeniser<TokenType, TokenisationErrorType>> = TokeniserBuilder::<TokenType, TokenisationErrorType>::new()
        .with_static_token(Regex::new(r"^end\b").unwrap(),      TokenType::End)
        .with_static_token(Regex::new(r"^do\b").unwrap(),       TokenType::Do)
        .with_static_token(Regex::new(r"^while\b").unwrap(),    TokenType::While)
        .with_static_token(Regex::new(r"^repeat\b").unwrap(),   TokenType::Repeat)
        .with_static_token(Regex::new(r"^until\b").unwrap(),    TokenType::Until)
        .with_static_token(Regex::new(r"^if\b").unwrap(),       TokenType::If)
        .with_static_token(Regex::new(r"^in\b").unwrap(),       TokenType::In)
        .with_static_token(Regex::new(r"^then\b").unwrap(),     TokenType::Then)
        .with_static_token(Regex::new(r"^elseif\b").unwrap(),   TokenType::Elseif)
        .with_static_token(Regex::new(r"^else\b").unwrap(),     TokenType::Else)
        .with_static_token(Regex::new(r"^for\b").unwrap(),      TokenType::For)
        .with_static_token(Regex::new(r"^function\b").unwrap(), TokenType::Function)
        .with_static_token(Regex::new(r"^local\b").unwrap(),    TokenType::Local)
        .with_static_token(Regex::new(r"^return\b").unwrap(),   TokenType::Return)
        .with_static_token(Regex::new(r"^break\b").unwrap(),    TokenType::Break)
        .with_static_token(Regex::new(r"^true\b").unwrap(),     TokenType::True)
        .with_static_token(Regex::new(r"^false\b").unwrap(),    TokenType::False)
        .with_static_token(Regex::new(r"^nil\b").unwrap(),      TokenType::Nil)
        .with_static_token(Regex::new(r"^and\b").unwrap(),      TokenType::And)
        .with_static_token(Regex::new(r"^or\b").unwrap(),       TokenType::Or)
        .with_static_token(Regex::new(r"^not\b").unwrap(),      TokenType::Not)
        .with_dynamic_token(
            Regex::new(r"^[a-zA-Z_]\w*").unwrap(),
            &parse_identifier
        )
        .with_static_token(Regex::new(r"^\.{3}").unwrap(),      TokenType::Varargs)
        .with_static_token(Regex::new(r"^\.{2}").unwrap(),      TokenType::Concat)
        .with_static_token(Regex::new(r"^={1}").unwrap(),       TokenType::Equals)
        .with_static_token(Regex::new(r"^={2}").unwrap(),       TokenType::DoubleEquals)
        .with_static_token(Regex::new(r"^:").unwrap(),          TokenType::Colon)
        .with_static_token(Regex::new(r"^,").unwrap(),          TokenType::Comma)
        .with_static_token(Regex::new(r"^\]").unwrap(),         TokenType::RightBracket)
        .with_static_token(Regex::new(r"^\(").unwrap(),         TokenType::LeftParenthesis)
        .with_static_token(Regex::new(r"^\)").unwrap(),         TokenType::RightParenthesis)
        .with_static_token(Regex::new(r"^\{").unwrap(),         TokenType::LeftBrace)
        .with_static_token(Regex::new(r"^\}").unwrap(),         TokenType::RightBrace)
        .with_static_token(Regex::new(r"^<<").unwrap(),         TokenType::LeftShift)
        .with_static_token(Regex::new(r"^>>").unwrap(),         TokenType::RightShift)
        .with_static_token(Regex::new(r"^&").unwrap(),          TokenType::BitwiseAnd)
        .with_static_token(Regex::new(r"^\|").unwrap(),         TokenType::BitwiseOr)
        .with_static_token(Regex::new(r"^~").unwrap(),          TokenType::BitwiseNeg)
        .with_static_token(Regex::new(r"^;").unwrap(),          TokenType::Semicolon)
        .with_static_token(Regex::new(r"^\+").unwrap(),         TokenType::Plus)
        .with_static_token(Regex::new(r"^\*").unwrap(),         TokenType::Multiply)
        .with_static_token(Regex::new(r"^/").unwrap(),          TokenType::Divide)
        .with_static_token(Regex::new(r"^\^").unwrap(),         TokenType::Power)
        .with_static_token(Regex::new(r"^%").unwrap(),          TokenType::Modulo)
        .with_static_token(Regex::new(r"^<=").unwrap(),         TokenType::LessEq)
        .with_static_token(Regex::new(r"^<").unwrap(),          TokenType::LessThan)
        .with_static_token(Regex::new(r"^>=").unwrap(),         TokenType::GreaterEq)
        .with_static_token(Regex::new(r"^>").unwrap(),          TokenType::GreaterThan)
        .with_static_token(Regex::new(r"^~=").unwrap(),         TokenType::NotEq)
        .with_static_token(Regex::new(r"^#").unwrap(),          TokenType::Length)
        .with_dynamic_token(
            Regex::new("^\"((\")|.)+?[^\\\\]\"").unwrap(),
            &parse_string
        )
        .with_dynamic_token(
            Regex::new(r"^'(\\'|.)+?[^\\]'").unwrap(),
            &parse_string
        )
        .with_complicated_token(
            Regex::new(r"^\[=*\[").unwrap(),
            &parse_multiline_string
        )
        .with_complicated_token(
            Regex::new(r"^-{2}\[=*\[").unwrap(),
            &parse_multiline_comment
        )
        .with_static_token(Regex::new(r"^-{2}.*").unwrap(),     TokenType::Comment)
        .with_static_token(Regex::new(r"^-").unwrap(),          TokenType::Minus)
        .with_static_token(Regex::new(r"\[").unwrap(),          TokenType::LeftBracket)
        .with_dynamic_token(
            Regex::new(r"^\d*").unwrap(),
            &parse_number
        )
        .with_static_token(Regex::new(r"^\.{1}").unwrap(),      TokenType::Dot)
        .with_eof_handler(&get_eof_token)
        .build();

    if tokeniser.is_some() {
        let mut t = tokeniser.unwrap();
        
        let tokens = t.tokenise(contents);
        tokens.map(|list| {
            for token in list {
                println!("{:?}", token);
            };
        }).or_else(|err| {
            println!("{:?}", err);
            Err(err)
        });
    }
}

fn parse_number(value: String, location: Location) -> Result<Token<TokenType>, TokenisationError<TokenType, TokenisationErrorType>> {
    Ok(Token {
        token_type: TokenType::NumberLiteral(1.0),
        location:   location
    })
}

fn parse_string(value: String, location: Location) -> Result<Token<TokenType>, TokenisationError<TokenType, TokenisationErrorType>> {
    Ok(Token {
        token_type: TokenType::StringLiteral(value),
        location:   location
    })
}

fn parse_identifier(
    value:    String,
    location: Location
) -> Result<Token<TokenType>, TokenisationError<TokenType, TokenisationErrorType>> {
    Ok(Token {
        token_type: TokenType::Identifier(value),
        location:   location
    })
}

fn parse_multiline_string(
    state:    &mut TokeniserState<TokenType, TokenisationErrorType>,
    location: Location
) -> Result<Token<TokenType>, TokenisationError<TokenType, TokenisationErrorType>> {
    parse_multiline(state).map(|parsed_str| {
        Token {
            token_type: TokenType::StringLiteral(parsed_str),
            location:   location
        }
    }).ok_or(
        TokenisationError {
            partial_token: get_eof_token(location),
            error_type:    TokenisationErrorType::UnfinishedLongString
        }
    )
}

fn parse_multiline_comment(
    state:    &mut TokeniserState<TokenType, TokenisationErrorType>,
    location: Location
) -> Result<Token<TokenType>, TokenisationError<TokenType, TokenisationErrorType>> {
    state.consume_chars(2);
    
    parse_multiline(state).map(|parsed_str| {
        Token {
            token_type: TokenType::Comment,
            location:   location
        }
    }).ok_or(
        TokenisationError {
            partial_token: get_eof_token(location),
            error_type:    TokenisationErrorType::UnfinishedLongComment
        }
    )
}

fn get_eof_token(location: Location) -> Token<TokenType> {
    Token {
        token_type: TokenType::EndOfFile,
        location:   location
    }
}

fn parse_multiline(
    tokeniser_state: &mut TokeniserState<TokenType, TokenisationErrorType>
) -> Option<String> {
    let mut line = tokeniser_state.line_buffer.as_ref().expect("Empty Line Buffer").clone();
    let start_mat = MULTILINE_START_REGEX.find(&line).expect("Failed to match the start of multiline regex");
    let depth = start_mat.end() - start_mat.start();
    let mut cols_to_undo = line.len();

    let end_mat = loop {
        let matches = MULTILINE_FINISH_REGEX.find_iter(&line);
    
        let end = matches
            .filter(|mat| (*mat).end() - (*mat).start() == depth)
            .into_iter()
            .next();

        if end.is_some() {
            break end;
        } else {
            if tokeniser_state.is_end_of_file() {
                break None;
            } else {
                cols_to_undo = line.len();
                tokeniser_state.pop_line();
                line = tokeniser_state.line_buffer.as_ref().expect("Empty Line Buffer").clone();
            }
        }
    };
    
    end_mat.map(|mat| {
        let value = String::from(&line[depth..mat.start()]);
        tokeniser_state.consume_chars(mat.end());
        tokeniser_state.location.col = mat.end() - cols_to_undo + 1;
        value
    })
}