mod error;

pub use error::ParseError;
pub use error::ParseErrorKind;

/// Creates userid needed for simple String-wrapping newtypes. The try_from parsing
/// validates the input string using a lmax length. We will have macros that support
/// more specific parsing functions in the future.
#[macro_export]
macro_rules! simple_string_newtype {
    ($n:ident($t:ty)) => {
        #[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
        pub struct $n($t);

        impl $n {
            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl AsRef<str> for $n {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }
        impl AsRef<$n> for $n {
            fn as_ref(&self) -> &$n {
                self
            }
        }

        impl std::convert::TryFrom<String> for $n {
            type Error = ParseError;
            fn try_from(value: String) -> std::result::Result<Self, ParseError> {
                value.parse()
            }
        }
        impl std::convert::TryFrom<&String> for $n {
            type Error = ParseError;
            fn try_from(value: &String) -> std::result::Result<Self, ParseError> {
                value.parse()
            }
        }
        impl std::convert::TryFrom<&str> for $n {
            type Error = ParseError;
            fn try_from(value: &str) -> std::result::Result<Self, ParseError> {
                value.parse()
            }
        }

        impl From<$n> for $t {
            #[inline]
            fn from(a: $n) -> $t {
                a.0
            }
        }
        impl From<&$n> for $t {
            #[inline]
            fn from(a: &$n) -> $t {
                a.0.clone()
            }
        }

        impl From<&$n> for $n {
            #[inline]
            fn from(a: &$n) -> $n {
                a.clone()
            }
        }

        impl std::fmt::Display for $n {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[inline]
pub fn from_str_is_only_ascii_lowercase_alpha(
    item: &str,
    s: &str,
    maxlen: usize,
) -> Result<String, ParseError> {
    if s.len() > maxlen {
        Err(ParseError {
            item: item.to_string(),
            value: s.to_string(),
            source: ParseErrorKind::LengthLimitExceeded(maxlen),
        })
    } else {
        if !s.chars().all(|c| c.is_ascii_lowercase()) {
            Err(ParseError {
                item: item.to_string(),
                value: s.to_string(),
                source: ParseErrorKind::InvalidSyntax(
                    "must only consist of ascii alpha characters".to_string(),
                ),
            })
        } else {
            Ok(s.to_string())
        }
    }
}
#[inline]
pub fn from_str_is_only_ascii_lowercase_alpha_or_dot(
    item: &str,
    s: &str,
    maxlen: usize,
) -> Result<String, ParseError> {
    if s.len() > maxlen {
        Err(ParseError {
            item: item.to_string(),
            value: s.to_string(),
            source: ParseErrorKind::LengthLimitExceeded(maxlen),
        })
    } else {
        if !s.chars().all(|c| c.is_ascii_lowercase() || c == '.') {
            Err(ParseError {
                item: item.to_string(),
                value: s.to_string(),
                source: ParseErrorKind::InvalidSyntax(
                    "must only consist of ascii alpha characters or .".to_string(),
                ),
            })
        } else {
            Ok(s.to_string())
        }
    }
}

#[inline]
pub fn from_str_is_only_ascii_lowercase_alphanum_or_hyphen(
    item: &str,
    s: &str,
    maxlen: usize,
) -> Result<String, ParseError> {
    if s.len() > maxlen {
        Err(ParseError {
            item: item.to_string(),
            value: s.to_string(),
            source: ParseErrorKind::LengthLimitExceeded(maxlen),
        })
    } else {
        if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Err(ParseError {
                item: item.to_string(),
                value: s.to_string(),
                source: ParseErrorKind::InvalidSyntax(
                    "must only consist of ascii alpha characters or -".to_string(),
                ),
            })
        } else {
            Ok(s.to_string())
        }
    }
}

#[inline]
pub fn from_str_is_only_ascii_lowercase_alphanum(
    item: &str,
    s: &str,
    maxlen: usize,
) -> Result<String, ParseError> {
    if s.len() > maxlen {
        Err(ParseError {
            item: item.to_string(),
            value: s.to_string(),
            source: ParseErrorKind::LengthLimitExceeded(maxlen),
        })
    } else {
        if !s.chars().all(|c| c.is_ascii_alphanumeric()) {
            Err(ParseError {
                item: item.to_string(),
                value: s.to_string(),
                source: ParseErrorKind::InvalidSyntax(
                    "must only consist of ascii alphanumeric characters".to_string(),
                ),
            })
        } else {
            Ok(s.to_string())
        }
    }
}

#[inline]
pub fn from_str_is_only_ascii_alpha_or_dot(
    item: &str,
    s: &str,
    maxlen: usize,
) -> Result<String, ParseError> {
    if s.len() > maxlen {
        Err(ParseError {
            item: item.to_string(),
            value: s.to_string(),
            source: ParseErrorKind::LengthLimitExceeded(maxlen),
        })
    } else {
        if !s.chars().all(|c| c.is_ascii() || c == '.') {
            Err(ParseError {
                item: item.to_string(),
                value: s.to_string(),
                source: ParseErrorKind::InvalidSyntax(
                    "must only consist of ascii alpha characters or .".to_string(),
                ),
            })
        } else {
            Ok(s.to_string())
        }
    }
}
