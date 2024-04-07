use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_break::parse_break;
use crate::root::nom_parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::nom_parser::parse_name::{parse_full_name, parse_simple_name, NameToken};
use nom::character::complete::{char, multispace0, multispace1};
use nom::sequence::Tuple;
use nom::Err::Error;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;

#[derive(Debug)]
pub struct InitialisationToken {
    location: Location,
    name: String,
    type_name: NameToken,
    value: EvaluableToken,
}

pub fn test_parse_initialisation<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match (tag("let"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_initialisation(x).map(|(s, x)| (s, LineTokens::Initialisation(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_initialisation(s: Span) -> ParseResult<Span, InitialisationToken> {
    let (s, l) = tag("let")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char(':')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, type_name) = parse_full_name(s)?;
    let (s, _) = multispace0(s)?;
    let (s, _) = char('=')(s)?;
    let (s, _) = multispace0(s)?;
    let (s, value) = parse_evaluable(s, true)?;

    Ok((
        s,
        InitialisationToken {
            location: Location::from_span(l),
            name: name.to_string(),
            type_name,
            value,
        },
    ))
}
