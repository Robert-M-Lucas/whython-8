use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_name::{parse_simple_name, UnresolvedNameToken};
use crate::root::parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use derive_getters::{Dissolve, Getters};
use nom::character::complete::{ satisfy};
use nom::sequence::Tuple;
use nom::Err::Error;
use nom::{IResult, Parser};
use nom_supreme::error::{BaseErrorKind, Expectation};
use nom_supreme::tag::complete::tag;
use substring::Substring;
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use crate::root::shared::types::TypeID;

#[derive(Debug, Getters, Dissolve)]
pub struct StructToken {
    location: Location,
    name: String,
    attributes: Parameters,
    id: Option<TypeID>
}

impl StructToken {
    pub fn set_id(&mut self, id: TypeID) {
        self.id = Some(id);
    }
}

pub fn test_parse_struct<'a>(s: Span<'a>) -> ParseResult<Span, ToplevelTestFn<'a>> {
    match (tag("struct"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_struct(x).map(|(s, x)| (s, TopLevelTokens::Struct(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_struct(s: Span) -> ParseResult<Span, StructToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("struct").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, contents) = default_section(s, '{')?;
    let (_, parameters) = parse_parameters(contents, None)?;

    Ok((
        s,
        StructToken {
            location,
            name: name.to_string(),
            attributes: parameters,
            id: None
        },
    ))
}
