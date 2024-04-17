use nom::character::complete::{char};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

use crate::root::parser::parse::{Location, ParseResult, Span};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct BreakToken {
    location: Location,
}

pub fn test_parse_break<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match tag("break")(s) {
        Ok(_) => Ok((s, |x| {
            parse_break(x).map(|(s, x)| (s, LineTokens::Break(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_break(s: Span) -> ParseResult<Span, BreakToken> {
    let (s, l) = tag("break")(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, _) = char(';')(s)?;
    Ok((
        s,
        BreakToken {
            location: Location::from_span(&l),
        },
    ))
}
