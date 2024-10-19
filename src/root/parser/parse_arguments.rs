use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_blocks::take_until_or_end_discard_smart;
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::discard_ignored;

/// Parses an argument list
pub fn parse_arguments<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, (), Vec<EvaluableToken>> {
    let mut s = s;
    let mut args = Vec::new();

    // Loop over arguments
    loop {
        let (ns, _) = discard_ignored(s)?;
        let (ns, section) = take_until_or_end_discard_smart(ns, ",")?;

        if section.is_empty() {
            break;
        }

        // Parse argument
        let res = parse_evaluable(section, containing_class, false)?.1;
        args.push(res);

        s = ns;
    }

    Ok(((), args))
}
