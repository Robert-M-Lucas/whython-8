use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use substring::Substring;

use crate::root::nom_parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::default_section;
use crate::root::nom_parser::parse_function::parse_line::{parse_lines, LineTokens};
use crate::root::nom_parser::parse_name::{parse_full_name, parse_simple_name, NameToken};
use crate::root::nom_parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::nom_parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

mod parse_assigner;
mod parse_assignment;
mod parse_break;
pub(crate) mod parse_evaluable;
mod parse_if;
mod parse_initialisation;
mod parse_line;
mod parse_literal;
mod parse_operator;
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
    match (tag("fn"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_function(x, None).map(|(s, x)| (s, TopLevelTokens::Function(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_function(s: Span, allow_self: Option<NameToken>) -> ParseResult<Span, FunctionToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("fn").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;

    let (s, contents) = default_section(s, '(')?;
    let (_, parameters) = parse_parameters(contents, allow_self)?;

    let (s, _) = discard_ignored(s)?;

    let (s, return_type) = if let Ok((s, _)) = tag::<_, _, ErrorTree>("->")(s) {
        let (s, _) = discard_ignored(s)?;
        let (s, return_type) = parse_full_name(s)?;
        (discard_ignored(s)?.0, Some(return_type))
    } else {
        (s, None)
    };

    let (s, contents) = default_section(s, '{')?;

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
