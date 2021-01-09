use std::fmt::{Display, Formatter};

use regex::Regex;
use lazy_static::lazy_static;

use crate::tokenisation::tokeniser::{Tokeniser, TokeniserState};
use crate::tokenisation::token::{Token, TokenData, Location};
use crate::tokenisation::builder::{TokeniserBuilder};
use crate::tokenisation::error::{TokenisationError, TokenisationErrorType};
use crate::parsing::{TerminalSymbol, NonterminalSymbol};

lazy_static!{
    pub static ref MULTILINE_START_REGEX:  Regex = Regex::new(r"^\[=*\[").unwrap();
    pub static ref MULTILINE_FINISH_REGEX: Regex = Regex::new(r"\]=*\]").unwrap();
}

pub enum LuaTokenData {
    Error(String),
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String)
}

impl TokenData for LuaTokenData {
    
}

impl Display for LuaTokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LuaTokenData::Error(val)           => write!(f, "{}",   val),
            LuaTokenData::Identifier(name)     => write!(f, "{}",   name),
            LuaTokenData::StringLiteral(value) => write!(f, "'{}'", value),
            LuaTokenData::NumberLiteral(value) => write!(f, "'{}'", value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LuaToken {
    End,
    Do,
    While,
    Repeat,
    Until,
    If,
    In,
    Then,
    Elseif,
    Else,
    For,
    Function,
    Local,
    Return,
    Break,
    True,
    False,
    Nil,
    And,
    Or,
    Not,
    Goto,
    Equals,
    DoubleEquals,
    Dot,
    Colon,
    DoubleColon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseNeg,
    Varargs,
    Semicolon,
    Plus,
    Minus,
    Multiply,
    Divide,
    FloorDivide,
    Power,
    Modulo,
    Concat,
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEq,
    NotEq,
    Length,
    Comment,
    EndOfFile,
    Error,
    Identifier,
    StringLiteral,
    NumberLiteral
}

impl TerminalSymbol for LuaToken {
    type DataType = LuaTokenData;

    fn get_name(&self) -> &'static str {
        match self {
            LuaToken::End              => "end",
            LuaToken::Do               => "do",
            LuaToken::While            => "while",
            LuaToken::Repeat           => "repeat",
            LuaToken::Until            => "until",
            LuaToken::If               => "if",
            LuaToken::In               => "in",
            LuaToken::Then             => "then",
            LuaToken::Elseif           => "elseif",
            LuaToken::Else             => "else",
            LuaToken::For              => "for",
            LuaToken::Function         => "function",
            LuaToken::Local            => "local",
            LuaToken::Return           => "return",
            LuaToken::Break            => "break",
            LuaToken::True             => "true",
            LuaToken::False            => "false",
            LuaToken::Nil              => "nil",
            LuaToken::And              => "and",
            LuaToken::Or               => "or",
            LuaToken::Not              => "not",
            LuaToken::Goto             => "goto",
            LuaToken::Equals           => "`=`",
            LuaToken::DoubleEquals     => "`==`",
            LuaToken::Dot              => "`.`",
            LuaToken::Colon            => "`:`",
            LuaToken::DoubleColon      => "`::`",
            LuaToken::Comma            => "`,`",
            LuaToken::LeftBracket      => "`[`",
            LuaToken::RightBracket     => "`]`",
            LuaToken::LeftParenthesis  => "`(`",
            LuaToken::RightParenthesis => "`)`",
            LuaToken::LeftBrace        => "`{`",
            LuaToken::RightBrace       => "`}`",
            LuaToken::LeftShift        => "`<<`",
            LuaToken::RightShift       => "`>>`",
            LuaToken::BitwiseAnd       => "`&`",
            LuaToken::BitwiseOr        => "`|`",
            LuaToken::BitwiseNeg       => "`~`",
            LuaToken::Varargs          => "`...`",
            LuaToken::Semicolon        => "`;`",
            LuaToken::Plus             => "`+`",
            LuaToken::Minus            => "`-`",
            LuaToken::Multiply         => "`*`",
            LuaToken::Divide           => "`/`",
            LuaToken::FloorDivide      => "`//`",
            LuaToken::Power            => "`^`",
            LuaToken::Modulo           => "`%`",
            LuaToken::Concat           => "`..`",
            LuaToken::LessThan         => "`<`",
            LuaToken::LessEq           => "`<=`",
            LuaToken::GreaterThan      => "`>`",
            LuaToken::GreaterEq        => "`>=`",
            LuaToken::NotEq            => "`~=`",
            LuaToken::Length           => "`#`",
            LuaToken::Comment          => "Comment",
            LuaToken::EndOfFile        => "<eof>",
            LuaToken::Error            => "Error",
            LuaToken::Identifier       => "Name",
            LuaToken::StringLiteral    => "LiteralString",
            LuaToken::NumberLiteral    => "Numeral"
        }
    }
}

impl Display for LuaToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            // LuaToken::Identifier(name)     => write!(f, "{}", name),
            // LuaToken::StringLiteral(value) => write!(f, "'{}'", value),
            // LuaToken::NumberLiteral(value) => write!(f, "'{}'", value),
            _                              => write!(f, "{:?}", self)
        }
    }
}

pub fn get_lua_tokeniser<'t>() -> Option<Tokeniser<'t, LuaToken, TokenisationErrorType>> {
    TokeniserBuilder::<LuaToken, TokenisationErrorType>::new()
        .with_static_token(Regex::new(r"^end\b").unwrap(),      LuaToken::End)
        .with_static_token(Regex::new(r"^do\b").unwrap(),       LuaToken::Do)
        .with_static_token(Regex::new(r"^while\b").unwrap(),    LuaToken::While)
        .with_static_token(Regex::new(r"^repeat\b").unwrap(),   LuaToken::Repeat)
        .with_static_token(Regex::new(r"^until\b").unwrap(),    LuaToken::Until)
        .with_static_token(Regex::new(r"^if\b").unwrap(),       LuaToken::If)
        .with_static_token(Regex::new(r"^in\b").unwrap(),       LuaToken::In)
        .with_static_token(Regex::new(r"^then\b").unwrap(),     LuaToken::Then)
        .with_static_token(Regex::new(r"^elseif\b").unwrap(),   LuaToken::Elseif)
        .with_static_token(Regex::new(r"^else\b").unwrap(),     LuaToken::Else)
        .with_static_token(Regex::new(r"^for\b").unwrap(),      LuaToken::For)
        .with_static_token(Regex::new(r"^function\b").unwrap(), LuaToken::Function)
        .with_static_token(Regex::new(r"^local\b").unwrap(),    LuaToken::Local)
        .with_static_token(Regex::new(r"^return\b").unwrap(),   LuaToken::Return)
        .with_static_token(Regex::new(r"^break\b").unwrap(),    LuaToken::Break)
        .with_static_token(Regex::new(r"^true\b").unwrap(),     LuaToken::True)
        .with_static_token(Regex::new(r"^false\b").unwrap(),    LuaToken::False)
        .with_static_token(Regex::new(r"^nil\b").unwrap(),      LuaToken::Nil)
        .with_static_token(Regex::new(r"^and\b").unwrap(),      LuaToken::And)
        .with_static_token(Regex::new(r"^or\b").unwrap(),       LuaToken::Or)
        .with_static_token(Regex::new(r"^not\b").unwrap(),      LuaToken::Not)
        .with_static_token(Regex::new(r"^goto\b").unwrap(),     LuaToken::Goto)
        .with_dynamic_token(
            Regex::new(r"^[a-zA-Z_]\w*").unwrap(),
            &parse_identifier
        )
        .with_static_token(Regex::new(r"^\.{3}").unwrap(),      LuaToken::Varargs)
        .with_static_token(Regex::new(r"^\.{2}").unwrap(),      LuaToken::Concat)
        .with_static_token(Regex::new(r"^==").unwrap(),         LuaToken::DoubleEquals)
        .with_static_token(Regex::new(r"^=").unwrap(),          LuaToken::Equals)
        .with_static_token(Regex::new(r"^::").unwrap(),         LuaToken::DoubleColon)
        .with_static_token(Regex::new(r"^:").unwrap(),          LuaToken::Colon)
        .with_static_token(Regex::new(r"^,").unwrap(),          LuaToken::Comma)
        .with_static_token(Regex::new(r"^\]").unwrap(),         LuaToken::RightBracket)
        .with_static_token(Regex::new(r"^\(").unwrap(),         LuaToken::LeftParenthesis)
        .with_static_token(Regex::new(r"^\)").unwrap(),         LuaToken::RightParenthesis)
        .with_static_token(Regex::new(r"^\{").unwrap(),         LuaToken::LeftBrace)
        .with_static_token(Regex::new(r"^\}").unwrap(),         LuaToken::RightBrace)
        .with_static_token(Regex::new(r"^<<").unwrap(),         LuaToken::LeftShift)
        .with_static_token(Regex::new(r"^>>").unwrap(),         LuaToken::RightShift)
        .with_static_token(Regex::new(r"^&").unwrap(),          LuaToken::BitwiseAnd)
        .with_static_token(Regex::new(r"^\|").unwrap(),         LuaToken::BitwiseOr)
        .with_static_token(Regex::new(r"^~").unwrap(),          LuaToken::BitwiseNeg)
        .with_static_token(Regex::new(r"^;").unwrap(),          LuaToken::Semicolon)
        .with_static_token(Regex::new(r"^\+").unwrap(),         LuaToken::Plus)
        .with_static_token(Regex::new(r"^\*").unwrap(),         LuaToken::Multiply)
        .with_static_token(Regex::new(r"^//").unwrap(),         LuaToken::FloorDivide)
        .with_static_token(Regex::new(r"^/").unwrap(),          LuaToken::Divide)
        .with_static_token(Regex::new(r"^\^").unwrap(),         LuaToken::Power)
        .with_static_token(Regex::new(r"^%").unwrap(),          LuaToken::Modulo)
        .with_static_token(Regex::new(r"^<=").unwrap(),         LuaToken::LessEq)
        .with_static_token(Regex::new(r"^<").unwrap(),          LuaToken::LessThan)
        .with_static_token(Regex::new(r"^>=").unwrap(),         LuaToken::GreaterEq)
        .with_static_token(Regex::new(r"^>").unwrap(),          LuaToken::GreaterThan)
        .with_static_token(Regex::new(r"^~=").unwrap(),         LuaToken::NotEq)
        .with_static_token(Regex::new(r"^#").unwrap(),          LuaToken::Length)
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
        .with_static_token(Regex::new(r"^-{2}.*").unwrap(),     LuaToken::Comment)
        .with_static_token(Regex::new(r"^-").unwrap(),          LuaToken::Minus)
        .with_static_token(Regex::new(r"\[").unwrap(),          LuaToken::LeftBracket)
        .with_dynamic_token(
            Regex::new(r"^\d+").unwrap(),
            &parse_number
        )
        .with_static_token(Regex::new(r"^\.{1}").unwrap(),      LuaToken::Dot)
        .with_error_handler('"',  &handle_unfinished_str)
        .with_error_handler('\'', &handle_unfinished_str)
        .with_eof_handler(&get_eof_token)
        .with_unexpected_symbol_handler(&get_unexpected_symbol_error)
        .build()
}

fn parse_number(value: String, location: Location) -> Result<Token<LuaToken>, TokenisationError<LuaToken, TokenisationErrorType>> {
    Ok(Token {
        token_type: LuaToken::NumberLiteral,
        token_data: Some(LuaTokenData::NumberLiteral(1.0)),
        location:   location
    })
}

fn parse_string(value: String, location: Location) -> Result<Token<LuaToken>, TokenisationError<LuaToken, TokenisationErrorType>> {
    let len = value.len() - 1;
    Ok(Token {
        token_type: LuaToken::StringLiteral,
        token_data: Some(LuaTokenData::StringLiteral(String::from(&value[1..len]))),
        location:   location
    })
}

fn parse_identifier(
    value:    String,
    location: Location
) -> Result<Token<LuaToken>, TokenisationError<LuaToken, TokenisationErrorType>> {
    Ok(Token {
        token_type: LuaToken::Identifier,
        token_data: Some(LuaTokenData::Identifier(value)),
        location:   location
    })
}

fn parse_multiline_string(
    state:    &mut TokeniserState<LuaToken, TokenisationErrorType>,
    location: Location
) -> Result<Token<LuaToken>, TokenisationError<LuaToken, TokenisationErrorType>> {
    parse_multiline(state)
        .map(|parsed_str| {
            Token {
                token_type: LuaToken::StringLiteral,
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
    state:    &mut TokeniserState<LuaToken, TokenisationErrorType>,
    location: Location
) -> Result<Token<LuaToken>, TokenisationError<LuaToken, TokenisationErrorType>> {
    state.consume_chars(2);

    parse_multiline(state)
        .map(|_| {
            Token {
                token_type: LuaToken::Comment,
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

fn get_eof_token(location: Location) -> Token<LuaToken> {
    Token {
        token_type: LuaToken::EndOfFile,
        token_data: None,
        location:   location
    }
}

fn parse_multiline(
    tokeniser_state: &mut TokeniserState<LuaToken, TokenisationErrorType>
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
) -> TokenisationError<LuaToken, TokenisationErrorType> {
    TokenisationError {
        partial_token: Token {
            token_type: LuaToken::StringLiteral,
            token_data: Some(LuaTokenData::Error(line)),
            location:   location
        },
        error_type: TokenisationErrorType::UnfinishedString
    }
}

fn get_unexpected_symbol_error(
    location: Location,
    symbol:   char
) -> TokenisationError<LuaToken, TokenisationErrorType> {
    TokenisationError {
        partial_token: Token {
            token_type: LuaToken::Error,
            token_data: Some(LuaTokenData::Error(symbol.to_string())),
            location:   location
        },
        error_type:    TokenisationErrorType::UnexpectedSymbol
    }
}
