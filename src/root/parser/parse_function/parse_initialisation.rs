use derive_getters::Getters;
use crate::root::parser::parse::{Location, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, FullNameWithIndirectionToken, parse_evaluable, parse_full_name};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use nom::character::complete::char;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;
use crate::root::parser::parse_name::{SimpleNameToken, parse_simple_name};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug, Getters)]
pub struct InitialisationToken {
    location: Location,
    name: SimpleNameToken,
    type_name: FullNameWithIndirectionToken,
    value: EvaluableToken,
}

pub fn test_parse_initialisation<'b>(s: Span<'_>) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (tag("let"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_initialisation(x, c).map(|(s, x)| (s, LineTokens::Initialisation(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_initialisation<'a>(s: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, Span<'a>, InitialisationToken> {
    let (s, l) = tag("let")(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, type_name) = parse_full_name(s, containing_class)?;
    let (s, _) = discard_ignored(s)?;
    let (s, _) = char('=')(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, value) = parse_evaluable(s, containing_class, true)?;

    Ok((
        s,
        InitialisationToken {
            location: Location::from_span(&l),
            name,
            type_name,
            value,
        },
    ))
}
