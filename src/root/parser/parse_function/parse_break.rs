use crate::root::parser::location::Location;
use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_util::discard_ignored;
use derive_getters::Getters;
use nom::character::complete::char;
use nom_supreme::tag::complete::tag;

/// Token representing a break
#[derive(Debug, Getters)]
pub struct BreakToken {
    location: Location,
}

/// Checks if the line should be parsed as a break
pub fn test_parse_break<'b>(s: Span<'_>) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match tag("break")(s) {
        Ok(_) => Ok((s, |x, _| {
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
