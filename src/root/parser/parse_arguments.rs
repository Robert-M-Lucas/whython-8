use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_blocks::take_until_discard_smart;
use crate::root::parser::parse_function::parse_evaluable::{
    parse_evaluable, EvaluableToken, EvaluableTokens,
};
use crate::root::parser::parse_name::SimpleNameToken;
use nom::bytes::complete::take_until;
use nom::InputTake;

pub fn parse_arguments<'a, 'b>(
    s: Span<'a>,
    containing_class: Option<&'b SimpleNameToken>,
) -> ParseResult<'a, (), Vec<EvaluableToken>> {
    let mut s = s;
    let mut args = Vec::new();
    let mut last = false;

    loop {
        // TODO: Account for brackets
        let (ns, section) = if let Ok((ns, section)) = take_until_discard_smart(s, ",") {
            (ns, section)
        } else {
            last = true;
            s.take_split(s.len())
        };

        let res = parse_evaluable(section, containing_class, false)?.1;

        if matches!(res.token(), EvaluableTokens::None) {
            if !last {
                todo!() // Expected evaluable
            }
        } else {
            args.push(res);
        }

        s = ns;
        if last {
            break;
        }
    }

    Ok(((), args))
}
