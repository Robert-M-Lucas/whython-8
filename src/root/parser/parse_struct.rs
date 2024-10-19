use crate::root::parser::location::Location;
use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_blocks::{parse_default_terminator_content, BRACE_TERMINATOR};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_parameters::{parse_parameters, Parameters};
use crate::root::parser::parse_toplevel::{TopLevelTokens, ToplevelTestFn};
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use crate::root::shared::common::TypeID;
use derive_getters::{Dissolve, Getters};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

/// Token representing a struct with location
#[derive(Debug, Getters, Dissolve)]
pub struct StructToken {
    location: Location,
    name: SimpleNameToken,
    attributes: Parameters,
    id: Option<TypeID>,
}

impl StructToken {
    /// Sets the ID of the struct
    pub fn set_id(&mut self, id: TypeID) {
        self.id = Some(id);
    }
}

/// Tests if line should be parsed as a struct
pub fn test_parse_struct(s: Span<'_>) -> ParseResult<Span, ToplevelTestFn<'_>> {
    match (tag("struct"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x| {
            parse_struct(x).map(|(s, x)| (s, TopLevelTokens::Struct(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parses line as a struct
pub fn parse_struct(s: Span) -> ParseResult<Span, StructToken> {
    let location = Location::from_span(&s);
    let (s, _) = tag("struct").parse(s)?;
    let (s, _) = require_ignored(s)?;
    let (s, name) = parse_simple_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, contents) = parse_default_terminator_content(s, &BRACE_TERMINATOR)?;
    // Parse attributes as parameters
    let (_, (attributes, _)) = parse_parameters(contents, None)?;

    Ok((
        s,
        StructToken {
            location,
            name,
            attributes,
            id: None,
        },
    ))
}
