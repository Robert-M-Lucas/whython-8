use crate::root::parser::location::Location;
use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_blocks::{parse_terminator_default_set, BRACE_TERMINATOR};
use crate::root::parser::parse_function::{parse_function, FunctionToken};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use derive_getters::{Dissolve, Getters};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

#[derive(Debug, Getters, Dissolve)]
pub struct ImplToken {
    location: Location,
    name: SimpleNameToken,
    functions: Vec<FunctionToken>,
}

pub fn test_parse_impl(s: Span<'_>) -> ParseResult<Span, ToplevelTestFn<'_>> {
    match (tag("impl"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_impl(x).map(|(s, x)| (s, TopLevelTokens::Impl(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_impl(s: Span) -> ParseResult<Span, ImplToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("impl").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, contents) = parse_terminator_default_set(s, &BRACE_TERMINATOR)?;

    let mut functions = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c)?;
        if cs.is_empty() {
            break;
        }

        let (cs, function) = parse_function(
            cs,
            // ? Pass class name (type) to function in case needed for self
            Some(&name),
        )?;

        functions.push(function);
        c = cs;
    }

    Ok((
        s,
        ImplToken {
            location,
            name,
            functions,
        },
    ))
}
