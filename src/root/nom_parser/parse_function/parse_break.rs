use nom::character::complete::char;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::nom_parser::parse_struct::parse_struct;
use crate::root::nom_parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub struct BreakToken {
    location: Location,
}

pub fn test_parse_break<'a>(s: Span) -> ParseResult<Span, LineTestFn<'a>> {
    if s.len() >= 5 && s.substring(0, 5) == "break" {
        Ok((s, |x| parse_break(x).map(|(s, b)| (s, LineTokens::Break(b)))))
    }
    else {
        Err(Error(
            TypeErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(
                    Expectation::Tag("break")
                ),
            }
        ))
    }
}

pub fn parse_break(s: Span) -> ParseResult<Span, BreakToken> {
    let (s, l) = tag("break")(s)?;
    let (s, _) = discard_ignored(s);
    let (s, _) = char(';')(s)?;
    Ok((s, BreakToken { location: Location::from_span(l) }))
}