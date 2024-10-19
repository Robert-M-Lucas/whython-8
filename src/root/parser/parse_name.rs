use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use derive_getters::Getters;
use nom::bytes::complete::take_till;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};

/// Token representing a simple name string with location
#[derive(Debug, Getters, Clone)]
pub struct SimpleNameToken {
    location: Location,
    name: String,
}

impl SimpleNameToken {
    pub fn new(s: Span) -> SimpleNameToken {
        SimpleNameToken {
            location: Location::from_span(&s),
            name: s.to_string(),
        }
    }

    pub fn new_builtin(s: String) -> SimpleNameToken {
        SimpleNameToken {
            location: Location::builtin(),
            name: s.to_string(),
        }
    }
    
    /// Gets the name out of the token
    pub fn take_name(self) -> String {
        self.name
    }
}

/// Parses a simple name string token
/// Allows alphabet and '_'
pub fn parse_simple_name(s: Span<'_>) -> ParseResult<'_, Span, SimpleNameToken> {
    // Allow alphabet and _
    let (s, n) = take_till(|c: char| c.is_whitespace() || (!c.is_alphabetic() && c != '_'))(s)?;

    // Disallow digit immediately after name (other symbols like operators are allowed)
    if let Some(first) = s.chars().next() {
        if first.is_ascii_digit() {
            return Err(Error(ErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(Expectation::Space),
            }));
        }
    }

    if n.is_empty() {
        Err(Error(ErrorTree::Base {
            location: s,
            kind: BaseErrorKind::Expected(Expectation::Alpha),
        }))
    } else {
        Ok((s, SimpleNameToken::new(n)))
    }
}
