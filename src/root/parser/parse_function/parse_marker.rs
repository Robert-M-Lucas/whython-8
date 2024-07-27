use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use derive_getters::Getters;
use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

#[derive(Debug, Getters)]
pub struct MarkerToken {
    value: String,
}

#[cfg(debug_assertions)]
pub fn test_parse_marker<'a, 'b>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a, 'b>> {
    match (char('@'), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, _| {
            parse_marker(x).map(|(s, x)| (s, LineTokens::Marker(x)))
        })),
        Err(e) => Err(e),
    }
}

#[cfg(debug_assertions)]
pub fn parse_marker(s: Span) -> ParseResult<Span, MarkerToken> {
    let (s, _) = char('@')(s)?;
    let (s, value) = take_till(|c| c == '\n' || c == '\r')(s)?;

    let (s, _) = discard_ignored(s)?;

    Ok((
        s,
        MarkerToken {
            value: value.fragment().to_string(),
        },
    ))
}
