use regex::Regex;
use lazy_static::lazy_static;

use crate::tokenisation::{Tokeniser, TokeniserState, Token, Location, TokeniserBuilder, TokenisationError, TokenisationErrorType};
use crate::lua::{LuaTerminal, LuaTokenData};

lazy_static!{
    pub static ref MULTILINE_START_REGEX:  Regex = Regex::new(r"^\[=*\[").unwrap();
    pub static ref MULTILINE_FINISH_REGEX: Regex = Regex::new(r"\]=*\]").unwrap();
}

pub fn get_lua_tokeniser<'t>() -> Option<Tokeniser<'t, LuaTerminal, TokenisationErrorType>> {
    TokeniserBuilder::<LuaTerminal, TokenisationErrorType>::new()
        .with_static_token(Regex::new(r"^end\b").unwrap(),      LuaTerminal::End)
        .with_static_token(Regex::new(r"^do\b").unwrap(),       LuaTerminal::Do)
        .with_static_token(Regex::new(r"^while\b").unwrap(),    LuaTerminal::While)
        .with_static_token(Regex::new(r"^repeat\b").unwrap(),   LuaTerminal::Repeat)
        .with_static_token(Regex::new(r"^until\b").unwrap(),    LuaTerminal::Until)
        .with_static_token(Regex::new(r"^if\b").unwrap(),       LuaTerminal::If)
        .with_static_token(Regex::new(r"^in\b").unwrap(),       LuaTerminal::In)
        .with_static_token(Regex::new(r"^then\b").unwrap(),     LuaTerminal::Then)
        .with_static_token(Regex::new(r"^elseif\b").unwrap(),   LuaTerminal::Elseif)
        .with_static_token(Regex::new(r"^else\b").unwrap(),     LuaTerminal::Else)
        .with_static_token(Regex::new(r"^for\b").unwrap(),      LuaTerminal::For)
        .with_static_token(Regex::new(r"^function\b").unwrap(), LuaTerminal::Function)
        .with_static_token(Regex::new(r"^local\b").unwrap(),    LuaTerminal::Local)
        .with_static_token(Regex::new(r"^return\b").unwrap(),   LuaTerminal::Return)
        .with_static_token(Regex::new(r"^break\b").unwrap(),    LuaTerminal::Break)
        .with_static_token(Regex::new(r"^true\b").unwrap(),     LuaTerminal::True)
        .with_static_token(Regex::new(r"^false\b").unwrap(),    LuaTerminal::False)
        .with_static_token(Regex::new(r"^nil\b").unwrap(),      LuaTerminal::Nil)
        .with_static_token(Regex::new(r"^and\b").unwrap(),      LuaTerminal::And)
        .with_static_token(Regex::new(r"^or\b").unwrap(),       LuaTerminal::Or)
        .with_static_token(Regex::new(r"^not\b").unwrap(),      LuaTerminal::Not)
        .with_static_token(Regex::new(r"^goto\b").unwrap(),     LuaTerminal::Goto)
        .with_dynamic_token(
            Regex::new(r"^[a-zA-Z_]\w*").unwrap(),
            &parse_identifier
        )
        .with_static_token(Regex::new(r"^\.{3}").unwrap(),      LuaTerminal::Varargs)
        .with_static_token(Regex::new(r"^\.{2}").unwrap(),      LuaTerminal::Concat)
        .with_static_token(Regex::new(r"^==").unwrap(),         LuaTerminal::DoubleEquals)
        .with_static_token(Regex::new(r"^=").unwrap(),          LuaTerminal::Equals)
        .with_static_token(Regex::new(r"^::").unwrap(),         LuaTerminal::DoubleColon)
        .with_static_token(Regex::new(r"^:").unwrap(),          LuaTerminal::Colon)
        .with_static_token(Regex::new(r"^,").unwrap(),          LuaTerminal::Comma)
        .with_static_token(Regex::new(r"^\]").unwrap(),         LuaTerminal::RightBracket)
        .with_static_token(Regex::new(r"^\(").unwrap(),         LuaTerminal::LeftParenthesis)
        .with_static_token(Regex::new(r"^\)").unwrap(),         LuaTerminal::RightParenthesis)
        .with_static_token(Regex::new(r"^\{").unwrap(),         LuaTerminal::LeftBrace)
        .with_static_token(Regex::new(r"^\}").unwrap(),         LuaTerminal::RightBrace)
        .with_static_token(Regex::new(r"^<<").unwrap(),         LuaTerminal::LeftShift)
        .with_static_token(Regex::new(r"^>>").unwrap(),         LuaTerminal::RightShift)
        .with_static_token(Regex::new(r"^&").unwrap(),          LuaTerminal::BitwiseAnd)
        .with_static_token(Regex::new(r"^\|").unwrap(),         LuaTerminal::BitwiseOr)
        .with_static_token(Regex::new(r"^~").unwrap(),          LuaTerminal::BitwiseNeg)
        .with_static_token(Regex::new(r"^;").unwrap(),          LuaTerminal::Semicolon)
        .with_static_token(Regex::new(r"^\+").unwrap(),         LuaTerminal::Plus)
        .with_static_token(Regex::new(r"^\*").unwrap(),         LuaTerminal::Multiply)
        .with_static_token(Regex::new(r"^//").unwrap(),         LuaTerminal::FloorDivide)
        .with_static_token(Regex::new(r"^/").unwrap(),          LuaTerminal::Divide)
        .with_static_token(Regex::new(r"^\^").unwrap(),         LuaTerminal::Power)
        .with_static_token(Regex::new(r"^%").unwrap(),          LuaTerminal::Modulo)
        .with_static_token(Regex::new(r"^<=").unwrap(),         LuaTerminal::LessEq)
        .with_static_token(Regex::new(r"^<").unwrap(),          LuaTerminal::LessThan)
        .with_static_token(Regex::new(r"^>=").unwrap(),         LuaTerminal::GreaterEq)
        .with_static_token(Regex::new(r"^>").unwrap(),          LuaTerminal::GreaterThan)
        .with_static_token(Regex::new(r"^~=").unwrap(),         LuaTerminal::NotEq)
        .with_static_token(Regex::new(r"^#").unwrap(),          LuaTerminal::Length)
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
        .with_static_token(Regex::new(r"^-{2}.*").unwrap(),     LuaTerminal::Comment)
        .with_static_token(Regex::new(r"^-").unwrap(),          LuaTerminal::Minus)
        .with_static_token(Regex::new(r"\[").unwrap(),          LuaTerminal::LeftBracket)
        .with_dynamic_token(
            Regex::new(r"^\d+").unwrap(),
            &parse_number
        )
        .with_static_token(Regex::new(r"^\.{1}").unwrap(),      LuaTerminal::Dot)
        .with_error_handler('"',  &handle_unfinished_str)
        .with_error_handler('\'', &handle_unfinished_str)
        .with_eof_handler(&get_eof_token)
        .with_unexpected_symbol_handler(&get_unexpected_symbol_error)
        .build()
}

fn parse_number(value: String, location: Location) -> Result<Token<LuaTerminal>, TokenisationError<LuaTerminal, TokenisationErrorType>> {
    Ok(Token {
        token_type: LuaTerminal::NumberLiteral,
        token_data: Some(LuaTokenData::NumberLiteral(1.0)),
        location:   location
    })
}

fn parse_string(value: String, location: Location) -> Result<Token<LuaTerminal>, TokenisationError<LuaTerminal, TokenisationErrorType>> {
    let len = value.len() - 1;
    Ok(Token {
        token_type: LuaTerminal::StringLiteral,
        token_data: Some(LuaTokenData::StringLiteral(String::from(&value[1..len]))),
        location:   location
    })
}

fn parse_identifier(
    value:    String,
    location: Location
) -> Result<Token<LuaTerminal>, TokenisationError<LuaTerminal, TokenisationErrorType>> {
    Ok(Token {
        token_type: LuaTerminal::Identifier,
        token_data: Some(LuaTokenData::Identifier(value)),
        location:   location
    })
}

fn parse_multiline_string(
    state:    &mut TokeniserState<LuaTerminal, TokenisationErrorType>,
    location: Location
) -> Result<Token<LuaTerminal>, TokenisationError<LuaTerminal, TokenisationErrorType>> {
    parse_multiline(state)
        .map(|parsed_str| {
            Token {
                token_type: LuaTerminal::StringLiteral,
                token_data: Some(LuaTokenData::StringLiteral(parsed_str)),
                location:   location
            }
        })
        .ok_or(
            TokenisationError {
                partial_token: get_eof_token(location),
                error_type:    TokenisationErrorType::UnfinishedLongString
            }
        )
}

fn parse_multiline_comment(
    state:    &mut TokeniserState<LuaTerminal, TokenisationErrorType>,
    location: Location
) -> Result<Token<LuaTerminal>, TokenisationError<LuaTerminal, TokenisationErrorType>> {
    state.consume_chars(2);

    parse_multiline(state)
        .map(|_| {
            Token {
                token_type: LuaTerminal::Comment,
                token_data: None,
                location:   location
            }
        })
        .ok_or(
            TokenisationError {
                partial_token: get_eof_token(location),
                error_type:    TokenisationErrorType::UnfinishedLongComment
            }
        )
}

fn get_eof_token(location: Location) -> Token<LuaTerminal> {
    Token {
        token_type: LuaTerminal::EndOfFile,
        token_data: None,
        location:   location
    }
}

fn parse_multiline(
    tokeniser_state: &mut TokeniserState<LuaTerminal, TokenisationErrorType>
) -> Option<String> {
    let mut line = tokeniser_state.line_buffer.as_ref().expect("Empty Line Buffer").clone();
    let start_mat = MULTILINE_START_REGEX.find(&line).expect("Failed to match the start of multiline regex");
    let depth = start_mat.end() - start_mat.start();
    let mut cols_to_undo = line.len();

    let end_mat = loop {
        let end = MULTILINE_FINISH_REGEX.find_iter(&line)
            .filter(|mat| (*mat).end() - (*mat).start() == depth)
            .into_iter()
            .next();

        if end.is_some() {
            break end;
        } else {
            if !tokeniser_state.has_next_line() {
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

fn handle_unfinished_str(
    location: Location,
    line:     String
) -> TokenisationError<LuaTerminal, TokenisationErrorType> {
    TokenisationError {
        partial_token: Token {
            token_type: LuaTerminal::StringLiteral,
            token_data: Some(LuaTokenData::Error(line)),
            location:   location
        },
        error_type: TokenisationErrorType::UnfinishedString
    }
}

fn get_unexpected_symbol_error(
    location: Location,
    symbol:   char
) -> TokenisationError<LuaTerminal, TokenisationErrorType> {
    TokenisationError {
        partial_token: Token {
            token_type: LuaTerminal::Error,
            token_data: Some(LuaTokenData::Error(symbol.to_string())),
            location:   location
        },
        error_type:    TokenisationErrorType::UnexpectedSymbol
    }
}
