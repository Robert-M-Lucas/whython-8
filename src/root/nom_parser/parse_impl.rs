use nom::Parser;
use nom_supreme::tag::complete::tag;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_fn::{FunctionToken, parse_function};
use crate::root::nom_parser::parse_name::parse_simple_name;
use crate::root::nom_parser::parse_parameters::parse_parameters;
use crate::root::nom_parser::parse_struct::StructToken;
use crate::root::nom_parser::parse_toplevel::TopLevelTokens;
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct ImplToken {
    location: Location,
    name: String,
    functions: Vec<FunctionToken>
}

pub fn parse_impl(s: Span) -> ParseResult<Span, ImplToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("impl").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s);
    let (s, contents) = braced_section(s)?;

    let mut functions = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c);
        if cs.is_empty() {
            break;
        }
        let (cs, function) = parse_function(cs)?;

        functions.push(function);
        c = cs;
    }

    Ok((
        s,
        ImplToken {
            location,
            name: name.to_string(),
            functions
        }
    ))
}