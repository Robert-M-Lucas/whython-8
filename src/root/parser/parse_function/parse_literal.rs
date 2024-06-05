use derive_getters::{Dissolve, Getters};
use crate::root::parser::parse::{Location, ParseResult, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use crate::root::builtin::int::IntType;
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::{Indirection, TypeID, TypeRef};
use crate::root::shared::types::Type;

#[derive(Debug, Dissolve, Getters)]
pub struct LiteralToken {
    location: Location,
    literal: LiteralTokens,
}

#[derive(Debug)]
pub enum LiteralTokens {
    Bool(bool),
    Int(i64),
}

impl LiteralTokens {
    pub fn default_type(&self) -> TypeRef {
        match self {
            LiteralTokens::Bool(_) => {
                todo!()
            }
            LiteralTokens::Int(_) => {
                TypeRef::new(IntType::id(), Indirection(0))
            }
        }
    }
}

pub fn parse_literal(s: Span) -> ParseResult<Span, LiteralToken> {
    let (s, _) = discard_ignored(s)?;

    let (ns, l) = alt((
        |x| tag("true")(x).map(|(s, _)| (s, LiteralTokens::Bool(true))),
        |x| tag("false")(x).map(|(s, _)| (s, LiteralTokens::Bool(false))),
        |x| nom::character::complete::i64(x).map(|(s, i)| (s, LiteralTokens::Int(i))),
    ))(s)?;

    let l = LiteralToken {
        location: Location::from_span(&s),
        literal: l,
    };

    Ok((ns, l))
}
