use nom::character::complete::char;
use nom_supreme::tag::complete::tag;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct ReturnToken {
    location: Location,
    return_value: EvaluableToken
}

pub fn parse_break(s: Span) -> ParseResult<Span, ReturnToken> {
    let (s, l) = tag("return")(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, value) = parse_evaluable(s, true)?;
    Ok((s, ReturnToken { location: Location::from_span(l), return_value: value }))
}