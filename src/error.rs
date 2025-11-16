use std::error::Error;
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum ParseErrorKind {
    #[error("maxlen is '{0}'")]
    LengthLimitExceeded(usize),
    #[error("invalid syntax: '{0}'")]
    InvalidSyntax(String),
    #[error("invalid syntax")]
    InvalidSyntaxWithInner(#[from] Box<dyn Error + Send + Sync>),
    #[error("invalid integer")]
    ParseInt(ParseIntError),
    #[error("missing field '{0}'")]
    MissingField(String),
    #[error("xxxxx => {0}")]
    Other(String),
}

#[derive(thiserror::Error, Debug)]
#[error("'{value}' has invalid syntax for {item} {source}")]
pub struct ParseError {
    pub(crate) item: String,
    pub(crate) value: String,
    // TODO why is source not printing stack trace - once it works also remove source from display above?
    pub(crate) source: ParseErrorKind,
}

impl ParseError {
    pub fn length_limit_exceeded(
        item: impl Into<String>,
        value: impl Into<String>,
        n: usize,
    ) -> ParseError {
        ParseError {
            item: item.into(),
            value: value.into(),
            source: ParseErrorKind::LengthLimitExceeded(n),
        }
    }
    pub fn invalid_syntax(
        item: impl Into<String>,
        value: impl Into<String>,
        msg: impl Into<String>,
    ) -> ParseError {
        ParseError {
            item: item.into(),
            value: value.into(),
            source: ParseErrorKind::InvalidSyntax(msg.into()),
        }
    }
    pub fn missing_field(
        item: impl Into<String>,
        value: impl Into<String>,
        field: impl Into<String>,
    ) -> ParseError {
        ParseError {
            item: item.into(),
            value: value.into(),
            source: ParseErrorKind::MissingField(field.into()),
        }
    }
    pub fn other(
        item: impl Into<String>,
        value: impl Into<String>,
        msg: impl Into<String>,
    ) -> ParseError {
        ParseError {
            item: item.into(),
            value: value.into(),
            source: ParseErrorKind::Other(msg.into()),
        }
    }
}
