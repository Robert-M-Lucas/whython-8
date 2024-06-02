use derive_getters::Getters;
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use substring::Substring;

use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_function::parse_line::{parse_lines, LineTokens};
use crate::root::parser::parse_name::{parse_full_name, parse_simple_name, UnresolvedNameToken};
use crate::root::parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

pub mod parse_assigner;
pub mod parse_assignment;
pub mod parse_break;
pub mod parse_evaluable;
pub mod parse_if;
pub mod parse_initialisation;
pub mod parse_line;
pub mod parse_literal;
pub mod parse_operator;
pub mod parse_return;
pub mod parse_while;

#[derive(Debug, Getters)]
pub struct FunctionToken {
    location: Location,
    name: String,
    return_type: Option<UnresolvedNameToken>,
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

pub fn parse_function(s: Span, allow_self: Option<UnresolvedNameToken>) -> ParseResult<Span, FunctionToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("fn").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let c_owned = allow_self.as_ref().and_then(|s| Some(s.base().to_string()));
    let containing_class = if let Some(s) = &c_owned {
        Some(s.as_str())
    } else { None };

    let (s, contents) = default_section(s, '(')?;
    let (_, parameters) = parse_parameters(contents, allow_self)?;

    let (s, _) = discard_ignored(s)?;

    let (s, return_type) = if let Ok((s, _)) = tag::<_, _, ErrorTree>("->")(s) {
        let (s, _) = discard_ignored(s)?;
        let (s, return_type) = parse_full_name(s, containing_class.and_then(|s| Some(s.to_string())))?;
        (discard_ignored(s)?.0, Some(return_type))
    } else {
        (s, None)
    };

    let (s, contents) = default_section(s, '{')?;

    let (_, lines) = parse_lines(contents, containing_class)?;

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
