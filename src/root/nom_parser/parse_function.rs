use nom::character::complete::{multispace0, multispace1};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use substring::Substring;

use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_blocks::{braced_section, bracketed_section};
use crate::root::nom_parser::parse_function::parse_line::{parse_lines, LineTokens};
use crate::root::nom_parser::parse_name::{parse_full_name, parse_simple_name, NameToken};
use crate::root::nom_parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::nom_parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};

pub mod base;
mod parse_break;
pub(crate) mod parse_evaluable;
mod parse_if;
mod parse_initialisation;
mod parse_line;
mod parse_return;
mod parse_while;

#[derive(Debug)]
pub struct FunctionToken {
    location: Location,
    name: String,
    return_type: Option<NameToken>,
    parameters: Parameters,
    lines: Vec<LineTokens>,
}

pub fn test_parse_function<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
    match (tag("fn"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_function(x).map(|(s, x)| (s, TopLevelTokens::Function(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_function(s: Span) -> ParseResult<Span, FunctionToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("fn").parse(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = multispace0(s)?;

    let (s, contents) = bracketed_section(s)?;
    let (_, parameters) = parse_parameters(contents)?;

    let (s, _) = multispace0(s)?;

    let (s, return_type) = if let Ok((s, _)) = tag::<_, _, TypeErrorTree>("->")(s) {
        let (s, _) = multispace0(s)?;
        let (s, return_type) = parse_full_name(s)?;
        (multispace0(s)?.0, Some(return_type))
    } else {
        (s, None)
    };

    let (s, contents) = braced_section(s)?;

    let (_, lines) = parse_lines(contents)?;

    Ok((
        s,
        FunctionToken {
            location,
            name: name.to_string(),
            return_type,
            parameters,
            lines,
        },
    ))
}
