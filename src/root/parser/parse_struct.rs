use crate::root::parser::parse::{Location, ParseResult, Span};
use crate::root::parser::parse_parameters::{Parameters, parse_parameters};
use crate::root::parser::parse_toplevel::{ToplevelTestFn, TopLevelTokens};
use derive_getters::{Dissolve, Getters};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use crate::root::shared::common::TypeID;

#[derive(Debug, Getters, Dissolve)]
pub struct StructToken {
    location: Location,
    name: SimpleNameToken,
    attributes: Parameters,
    id: Option<TypeID>
}

impl StructToken {
    pub fn set_id(&mut self, id: TypeID) {
        self.id = Some(id);
    }
}

pub fn test_parse_struct(s: Span<'_>) -> ParseResult<Span, ToplevelTestFn<'_>> {
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
    let (_, (parameters, _)) = parse_parameters(contents, None)?;

    Ok((
        s,
        StructToken {
            location,
            name,
            attributes: parameters,
            id: None
        },
    ))
}
