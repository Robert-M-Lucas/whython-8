use crate::root::builtin::types::bool::BoolType;
use crate::root::builtin::types::int::IntType;
use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::TypeID;
use derive_getters::{Dissolve, Getters};
use nom::branch::alt;
use nom::bytes::complete::tag;
use crate::root::parser::location::Location;

#[derive(Debug, Dissolve, Getters)]
pub struct LiteralToken {
    location: Location,
    literal: LiteralTokens,
}

#[derive(Debug)]
pub enum LiteralTokens {
    Bool(bool),
    Int(i128),
}

impl LiteralTokens {
    pub fn default_type(&self) -> TypeID {
        match self {
            LiteralTokens::Bool(_) => BoolType::id(),
            LiteralTokens::Int(_) => IntType::id(),
        }
    }
}

pub fn parse_literal(s: Span) -> ParseResult<Span, LiteralToken> {
    let (s, _) = discard_ignored(s)?;

    let (ns, l) = alt((
        |x| tag("true")(x).map(|(s, _)| (s, LiteralTokens::Bool(true))),
        |x| tag("false")(x).map(|(s, _)| (s, LiteralTokens::Bool(false))),
        |x| nom::character::complete::i128(x).map(|(s, i)| (s, LiteralTokens::Int(i))),
    ))(s)?;

    let l = LiteralToken {
        location: Location::from_span(&s),
        literal: l,
    };

    Ok((ns, l))
}
