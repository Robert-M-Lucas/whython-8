#[cfg(debug_assertions)]
use derive_getters::Getters;
#[cfg(debug_assertions)]
use nom::bytes::complete::take_till;
#[cfg(debug_assertions)]
use nom::character::complete::char;
#[cfg(debug_assertions)]
use nom::sequence::Tuple;

#[cfg(debug_assertions)]
use crate::root::parser::parse::{ParseResult, Span};
#[cfg(debug_assertions)]
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
#[cfg(debug_assertions)]
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

/// Debug marker token
#[cfg(debug_assertions)]
#[derive(Debug, Getters)]
pub struct MarkerToken {
    value: String,
}

/// Test if line should be parsed as a marker
#[cfg(debug_assertions)]
pub fn test_parse_marker<'b>(s: Span) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (char('@'), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, _| {
            parse_marker(x).map(|(s, x)| (s, LineTokens::Marker(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parse marker
#[cfg(debug_assertions)]
pub fn parse_marker(s: Span) -> ParseResult<Span, MarkerToken> {
    let (s, _) = char('@')(s)?;
    // Get marker text until newline
    let (s, value) = take_till(|c| c == '\n' || c == '\r')(s)?;

    let (s, _) = discard_ignored(s)?;

    Ok((
        s,
        MarkerToken {
            value: value.fragment().to_string(),
        },
    ))
}
