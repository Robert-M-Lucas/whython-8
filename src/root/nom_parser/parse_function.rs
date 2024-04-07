pub mod base;
mod parse_line;
mod parse_break;
pub(crate) mod parse_evaluable;
mod parse_return;
mod parse_initialisation;
mod parse_while;

use nom::character::complete::char;
use nom::Err::Error;
use nom_supreme::tag::complete::tag;
use nom::Parser;
use nom_supreme::error::{BaseErrorKind, Expectation};
use substring::Substring;
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_blocks::{braced_section, bracketed_section};
use parse_line::parse_line;
use crate::root::nom_parser::parse_function::parse_line::{LineTokens, parse_lines};
use crate::root::nom_parser::parse_impl::parse_impl;
use crate::root::nom_parser::parse_name::{NameToken, parse_full_name, parse_simple_name};
use crate::root::nom_parser::parse_parameters::{Parameters, parse_parameters};
use crate::root::nom_parser::parse_struct::StructToken;
use crate::root::nom_parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct FunctionToken {
    location: Location,
    name: String,
    return_type: Option<NameToken>,
    parameters: Parameters,
    lines: Vec<LineTokens>
}

pub fn test_parse_function<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
    match tag::<_, _, TypeErrorTree<'a>>("fn")(s) {
        Ok(_) => Ok((s, |x| parse_function(x).map(|(s, x)| (s, TopLevelTokens::Function(x))))),
        Err(e) => Err(e)
    }
}

pub fn parse_function(s: Span) -> ParseResult<Span, FunctionToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("fn").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s);

    let (s, contents) = bracketed_section(s)?;
    let (_, parameters) = parse_parameters(contents)?;

    let (s, _) = discard_ignored(s);

    let (s, return_type) = if let Ok((s, _)) = tag::<&str, Span, TypeErrorTree>("->")(s) {
        let (s, _) = discard_ignored(s);
        let (s, return_type) = parse_full_name(s)?;
        (discard_ignored(s).0, Some(return_type))
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
        }
    ))
}

