use nom::character::complete::multispace0;
use nom::character::streaming::multispace1;
use nom::Parser;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_function::{FunctionToken, parse_function};
use crate::root::nom_parser::parse_name::parse_simple_name;
use crate::root::nom_parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};

#[derive(Debug)]
pub struct ImplToken {
    location: Location,
    name: String,
    functions: Vec<FunctionToken>
}

pub fn test_parse_impl<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
    match (tag("impl"), multispace1).parse(s) {
        Ok(_) => Ok((s, |x| parse_impl(x).map(|(s, x)| (s, TopLevelTokens::Impl(x))))),
        Err(e) => Err(e)
    }
}

pub fn parse_impl(s: Span) -> ParseResult<Span, ImplToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("impl").parse(s)?;
    let (s, _) = multispace1(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = multispace0(s)?;
    let (s, contents) = braced_section(s)?;

    let mut functions = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = multispace0(c)?;
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