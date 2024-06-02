use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_function::parse_break::parse_break;
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::parser::parse_name::{parse_full_name, parse_simple_name, UnresolvedNameToken};
use nom::character::complete::{char};
use nom::sequence::Tuple;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct InitialisationToken {
    location: Location,
    name: String,
    type_name: UnresolvedNameToken,
    value: EvaluableToken,
}

pub fn test_parse_initialisation<'a, 'b>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a, 'b>> {
    match (tag("let"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_initialisation(x, c).map(|(s, x)| (s, LineTokens::Initialisation(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_initialisation<'a, 'b>(s: Span<'a>, containing_class: Option<&'b str>) -> ParseResult<'a, Span<'a>, InitialisationToken> {
    let (s, l) = tag("let")(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, type_name) = parse_full_name(s, containing_class.and_then(|s| Some(s.to_string())))?;
    let (s, _) = discard_ignored(s)?;
    let (s, _) = char('=')(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, value) = parse_evaluable(s, containing_class, true)?;

    Ok((
        s,
        InitialisationToken {
            location: Location::from_span(&l),
            name: name.to_string(),
            type_name,
            value,
        },
    ))
}
