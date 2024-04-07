use nom::character::complete::{char, multispace0, multispace1};
use nom::Parser;
use nom_supreme::tag::complete::tag;
use nom::sequence::Tuple;

use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};

#[derive(Debug)]
pub struct BreakToken {
    location: Location,
}

pub fn test_parse_break<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match (tag("break"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| parse_break(x).map(|(s, x)| (s, LineTokens::Break(x))))),
        Err(e) => Err(e)
    }
}

pub fn parse_break(s: Span) -> ParseResult<Span, BreakToken> {
    let (s, l) = tag("break")(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(';')(s)?;
    Ok((s, BreakToken { location: Location::from_span(l) }))
}