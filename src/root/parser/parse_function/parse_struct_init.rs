use derive_getters::{Dissolve, Getters};
use nom::bytes::complete::tag;
use nom::character::streaming::char;

use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_blocks::{
    parse_terminator_default_set, take_until_or_end_discard_smart, BRACE_TERMINATOR,
};
use crate::root::parser::parse_function::parse_evaluable::{
    parse_evaluable, parse_full_name, EvaluableToken, FullNameWithIndirectionToken,
};
use crate::root::parser::parse_name::{parse_simple_name, SimpleNameToken};
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::Indirection;

#[derive(Debug, Dissolve, Getters)]
pub struct StructInitToken {
    location: Location,
    name: FullNameWithIndirectionToken,
    heap_alloc: bool,
    contents: Vec<(SimpleNameToken, EvaluableToken)>,
}

pub fn parse_struct_init<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, StructInitToken> {
    let (s, _) = discard_ignored(s)?;

    let (s, heap_alloc) = tag::<&str, Span, ErrorTree>("new")(s)
        .map(|(ns, _)| (ns, true))
        .unwrap_or((s, false));

    let (s, struct_name) = parse_full_name(s, containing_class)?;
    debug_assert!(*struct_name.indirection() == Indirection(0)); // TODO

    let (s, _) = discard_ignored(s)?;

    let (remaining, in_braces) = parse_terminator_default_set(s, &BRACE_TERMINATOR)?;

    let mut contents = Vec::new();

    let mut s = discard_ignored(in_braces)?.0;
    while !s.is_empty() {
        let ns = s;
        let (ns, _) = discard_ignored(ns)?;
        let (ns, name) = parse_simple_name(ns)?;
        let (ns, _) = discard_ignored(ns)?;
        let (ns, _) = char(':')(ns)?;
        let (ns, _) = discard_ignored(ns)?;

        let (ns, to_eval) = take_until_or_end_discard_smart(ns, ",")?;
        let (_, eval) = parse_evaluable(to_eval, containing_class, false)?;
        contents.push((name, eval));
        s = ns;
    }

    Ok((
        remaining,
        StructInitToken {
            location: struct_name.inner().location().clone(),
            name: struct_name,
            heap_alloc,
            contents,
        },
    ))
}
