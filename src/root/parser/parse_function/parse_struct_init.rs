use derive_getters::{Dissolve, Getters};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::bytes::streaming::take_until;
use nom::character::streaming::char;
use crate::root::parser::parse::{Location, ParseResult, Span};
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens, FullNameToken, FullNameWithIndirectionToken, parse_evaluable, parse_full_name};
use crate::root::parser::parse_function::parse_literal::{LiteralToken, LiteralTokens};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_parameters::parse_parameters;
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::Indirection;

#[derive(Debug, Dissolve, Getters)]
pub struct StructInitToken {
    location: Location,
    name: FullNameWithIndirectionToken,
    contents: Vec<(SimpleNameToken, EvaluableToken)>,
}

pub fn parse_struct_init<'a, 'b>(s: Span<'a>, containing_class: Option<&'b SimpleNameToken>) -> ParseResult<'a, Span<'a>, StructInitToken> {
    let (s, _) = discard_ignored(s)?;

    let (s, struct_name) = parse_full_name(s, containing_class.clone())?;
    debug_assert!(*struct_name.indirection() == Indirection(0)); // TODO

    let (s, _) = discard_ignored(s)?;

    let (remaining, in_braces) = default_section( s,'{')?;

    let mut contents = Vec::new();

    let mut s = discard_ignored(in_braces)?.0;
    while !s.is_empty() {
        let ns = s;
        let (ns, _) = discard_ignored(ns)?;
        let (ns, name) = parse_simple_name(ns)?;
        let (ns, _) = discard_ignored(ns)?;
        let (ns, _) = char(':')(ns)?;
        let (ns, _) = discard_ignored(ns)?;

        let (ns, to_eval) = take_till(|c| c == ',')(ns)?;
        let ns = if ns.is_empty() { ns } else { char(',')(ns)?.0 };
        let (_, eval) = parse_evaluable(to_eval, containing_class, false)?;
        contents.push((name, eval));
        s = ns;
    }

    Ok((remaining, StructInitToken {
        location: struct_name.inner().location().clone(),
        name: struct_name,
        contents
    }))
}