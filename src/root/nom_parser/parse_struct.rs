use nom::Err::Error;
use nom_supreme::tag::complete::tag;
use nom::Parser;
use nom_supreme::error::{BaseErrorKind, Expectation};
use crate::root::nom_parser::parse::{Location, ParseResult, Span, TypeErrorTree};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_name::{NameToken, parse_simple_name};
use crate::root::nom_parser::parse_parameters::{Parameters, parse_parameters};
use crate::root::nom_parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct StructToken {
    location: Location,
    name: String,
    attributes: Parameters
}

pub fn test_parse_struct<'a>(s: Span) -> ParseResult<Span, ToplevelTestFn<'a>> {
    if s.len() >= 6 && &s[..6] == "struct" {
        Ok((s, |x| parse_struct(x).map(|(s, st)| (s, TopLevelTokens::Struct(st)))))
    }
    else {
        Err(Error(
            TypeErrorTree::Base {
                location: s,
                kind: BaseErrorKind::Expected(
                    Expectation::Tag("struct")
                ),
            }
        ))
    }
}

pub fn parse_struct(s: Span) -> ParseResult<Span, StructToken> {
    let location = Location::from_span(s);
    let (s, _) = tag("struct").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s);
    let (s, contents) = braced_section(s)?;
    let (_, parameters) = parse_parameters(contents)?;

    Ok((
        s,
        StructToken {
            location,
            name: name.to_string(),
            attributes: parameters
        }
        ))
}
