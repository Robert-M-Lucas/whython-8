use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_blocks::{take_until_discard_smart, take_until_or_end_discard_smart};
use crate::root::parser::parse_function::parse_evaluable::{
    parse_evaluable, EvaluableToken, EvaluableTokens,
};
use crate::root::parser::parse_name::SimpleNameToken;
use nom::bytes::complete::take_until;
use nom::InputTake;
use crate::root::parser::parse_util::discard_ignored;

pub fn parse_arguments<'a, 'b>(
    s: Span<'a>,
    containing_class: Option<&'b SimpleNameToken>,
) -> ParseResult<'a, (), Vec<EvaluableToken>> {
    let mut s = s;
    println!("ARGS SPAN - {}", s);
    let mut args = Vec::new();

    loop {
        let (ns, _) = discard_ignored(s)?;
        let (ns, section) = take_until_or_end_discard_smart(ns, ",")?;

        if section.is_empty() {
            break;
        }

        let res = parse_evaluable(section, containing_class, false)?.1;
        args.push(res);

        s = ns;
    }

    println!("{}", args.len());

    Ok(((), args))
}
