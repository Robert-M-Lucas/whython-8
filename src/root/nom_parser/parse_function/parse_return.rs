use nom::character::complete::char;
use nom::character::streaming::multispace1;
use nom::Err::Error;
use nom::sequence::Tuple;
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_function::parse_break::parse_break;
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};

#[derive(Debug)]
pub struct ReturnToken {
    location: Location,
    return_value: EvaluableToken
}

pub fn test_parse_return<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match (tag("return"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| parse_return(x).map(|(s, x)| (s, LineTokens::Return(x))))),
        Err(e) => Err(e)
    }
}

pub fn parse_return(s: Span) -> ParseResult<Span, ReturnToken> {
    let (s, l) = tag("return")(s)?;
    let (s, _) = multispace1(s)?;
    let (s, value) = parse_evaluable(s, true)?;
    Ok((s, ReturnToken { location: Location::from_span(l), return_value: value }))
}