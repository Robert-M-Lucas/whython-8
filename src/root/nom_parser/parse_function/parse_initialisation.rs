use nom::character::complete::char;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::nom_parser::parse_name::{NameToken, parse_full_name, parse_simple_name};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};


#[derive(Debug)]
pub struct InitialisationToken {
    location: Location,
    name: String,
    type_name: NameToken,
    value: EvaluableToken
}

pub fn test_parse_initialisation<'a>(s: Span) -> ParseResult<Span, LineTestFn<'a>> {
    if s.len() >= 3 && s.substring(0, 3) == "let" {
        Ok((s, |x| parse_initialisation(x).map(|(s, i)| (s, LineTokens::Initialisation(i)))))
    }
    else {
        Err(Error(
            TypeErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(
                    Expectation::Tag("let")
                ),
            }
        ))
    }
}

pub fn parse_initialisation(s: Span) -> ParseResult<Span, InitialisationToken> {
    let (s, l) = tag("let")(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s);
    let (s, _) = char(':')(s)?;
    let (s, _) = discard_ignored(s);
    let (s, type_name) = parse_full_name(s)?;
    let (s, _) = discard_ignored(s);
    let (s, _) = char('=')(s)?;
    let (s, _) = discard_ignored(s);
    let (s, value) = parse_evaluable(s, true)?;

    Ok((s, InitialisationToken {
        location: Location::from_span(l),
        name: name.to_string(),
        type_name,
        value,
    }))
}
