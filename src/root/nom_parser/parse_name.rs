use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_evaluable::EvaluableToken;
use nom::bytes::complete::take_till;
use nom::Err::Error;
use nom::InputTake;
use nom_supreme::error::{BaseErrorKind, Expectation};

#[derive(Debug)]
enum NameConnectors {
    NonStatic,
    Static,
}

#[derive(Debug)]
pub struct NameToken {
    location: Location,
    base: String,
    names: Vec<(NameConnectors, String)>,
    function_call: Option<Vec<EvaluableToken>>,
}

pub fn parse_full_name(s: Span) -> ParseResult<Span, NameToken> {
    // TODO: Handle function calls

    let location = Location::from_span(s);

    let (mut s, base_name) = parse_simple_name(s)?;
    let mut names = Vec::new();

    loop {
        let ns;
        let connector = if let Some(next) = s.chars().next() {
            if next == '.' {
                ns = s.take_split(1).0;
                NameConnectors::NonStatic
            } else if next == ':' && s.chars().nth(1).is_some_and(|c| c == ':') {
                ns = s.take_split(2).0;
                NameConnectors::Static
            } else {
                break;
            }
        } else {
            break;
        };

        let (ns, name) = parse_simple_name(ns)?;

        names.push((connector, name.to_string()));

        s = ns;
    }

    Ok((
        s,
        NameToken {
            location,
            base: base_name.to_string(),
            names,
            function_call: None,
        },
    ))
}

pub fn parse_simple_name(s: Span) -> ParseResult {
    let (s, n) = take_till(|c: char| c.is_whitespace() || (!c.is_alphabetic() && c != '_'))(s)?;

    if let Some(first) = s.chars().next() {
        if first.is_ascii_digit() {
            return Err(Error(TypeErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(Expectation::Space),
            }));
        }
    }

    if n.is_empty() {
        Err(Error(TypeErrorTree::Base {
            location: s,
            kind: BaseErrorKind::Expected(Expectation::Alpha),
        }))
    } else {
        Ok((s, n))
    }
}
