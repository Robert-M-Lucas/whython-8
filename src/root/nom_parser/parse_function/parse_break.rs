use nom::character::complete::char;
use nom_supreme::tag::complete::tag;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub struct BreakToken {
    location: Location,
}

pub fn parse_break(s: Span) -> ParseResult<Span, BreakToken> {
    let (s, l) = tag("break")(s)?;
    let (s, _) = discard_ignored(s);
    let (s, _) = char(';')(s)?;
    Ok((s, BreakToken { location: Location::from_span(l) }))
}