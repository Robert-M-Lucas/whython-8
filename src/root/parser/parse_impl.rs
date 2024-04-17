use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

use crate::root::parser::parse::{Location, ParseResult, Span};
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_function::{parse_function, FunctionToken};
use crate::root::parser::parse_name::{NameToken, parse_simple_name};
use crate::root::parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct ImplToken {
    location: Location,
    name: String,
    functions: Vec<FunctionToken>,
}

pub fn test_parse_impl<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
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
    let (s, contents) = default_section(s, '{')?;

    let mut functions = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c)?;
        if cs.is_empty() {
            break;
        }
        let (cs, function) = parse_function(cs, Some(NameToken::from_simple(&name)))?;

        functions.push(function);
        c = cs;
    }

    Ok((
        s,
        ImplToken {
            location,
            name: name.to_string(),
            functions,
        },
    ))
}
