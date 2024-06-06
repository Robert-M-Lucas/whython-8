use nom::bytes::complete::take_till;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use derive_getters::Getters;
use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};

#[derive(Debug, Getters, Clone)]
pub struct SimpleNameToken {
    location: Location,
    name: String
}

impl SimpleNameToken {
    pub fn new(s: Span) -> SimpleNameToken {
        SimpleNameToken {
            location: Location::from_span(&s),
            name: s.to_string()
        }
    }

    pub fn new_builtin(s: String) -> SimpleNameToken {
        SimpleNameToken {
            location: Location::builtin(),
            name: s.to_string()
        }
    }
}

pub fn parse_simple_name<'a>(s: Span<'a>) -> ParseResult<'a, Span, SimpleNameToken> {
    let (s, n) = take_till(|c: char| c.is_whitespace() || (!c.is_alphabetic() && c != '_'))(s)?;

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


