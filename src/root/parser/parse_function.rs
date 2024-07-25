use derive_getters::{Dissolve, Getters};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_blocks::{BRACE_TERMINATOR, BRACKET_TERMINATOR, parse_terminator_default_set};
use crate::root::parser::parse_function::parse_evaluable::{FullNameWithIndirectionToken, parse_full_name};
use crate::root::parser::parse_function::parse_line::{LineTokens, parse_lines};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_parameters::{Parameters, parse_parameters};
use crate::root::parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};
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
mod parse_struct_init;
mod parse_marker;

#[derive(Debug, Getters, Dissolve)]
pub struct FunctionToken {
    location: Location,
    end_location: Location,
    name: SimpleNameToken,
    return_type: Option<FullNameWithIndirectionToken>,
    dynamic: bool,
    parameters: Parameters,
    lines: Vec<LineTokens>,
}

pub fn test_parse_function(s: Span<'_>) -> ParseResult<Span, ToplevelTestFn<'_>> {
    match (tag("fn"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_function(x, None).map(|(s, x)| (s, TopLevelTokens::Function(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_function<'a, 'b>(s: Span<'a>, allow_self: Option<&'b SimpleNameToken>) -> ParseResult<'a, Span<'a>, FunctionToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("fn").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    // let c_owned = allow_self.as_ref().and_then(|s| Some(s.base().to_string()));
    // let containing_class = if let Some(s) = &c_owned {
    //     Some(s.as_str())
    // } else { None };

    let (s, contents) = parse_terminator_default_set(s, &BRACKET_TERMINATOR)?;
    let (_, (parameters, has_self)) = parse_parameters(contents, allow_self)?;

    let (s, _) = discard_ignored(s)?;

    let (s, return_type) = if let Ok((s, _)) = tag::<_, _, ErrorTree>("->")(s) {
        let (s, _) = discard_ignored(s)?;
        let location = Location::from_span(&s);
        let (s, return_type) = parse_full_name(s, allow_self)?;
        (discard_ignored(s)?.0, Some(return_type))
    } else {
        (s, None)
    };

    let (s, contents) = parse_terminator_default_set(s, &BRACE_TERMINATOR)?;

    let end_location = Location::from_span_end(&contents);

    let (_, lines) = parse_lines(contents, allow_self)?;



    Ok((
        s,
        FunctionToken {
            dynamic: has_self,
            location,
            end_location,
            name,
            return_type,
            parameters,
            lines,
        },
    ))
}
