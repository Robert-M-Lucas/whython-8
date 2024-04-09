use crate::root::nom_parser::parse::{Location, ParseResult, Span, ErrorTree};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_name::{parse_simple_name, NameToken};
use crate::root::nom_parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::nom_parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use nom::character::complete::{multispace0, multispace1, satisfy};
use nom::sequence::Tuple;
use nom::Err::Error;
use nom::{IResult, Parser};
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;

#[derive(Debug)]
pub struct StructToken {
    location: Location,
    name: String,
    attributes: Parameters,
}

pub fn test_parse_struct<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
    match (tag("struct"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_struct(x).map(|(s, x)| (s, TopLevelTokens::Struct(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_struct(s: Span) -> ParseResult<Span, StructToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("struct").parse(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = multispace0(s)?;
    let (s, contents) = braced_section(s)?;
    let (_, parameters) = parse_parameters(contents)?;

    Ok((
        s,
        StructToken {
            location,
            name: name.to_string(),
            attributes: parameters,
        },
    ))
}
