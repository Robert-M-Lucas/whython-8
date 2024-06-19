use nom::bytes::complete::take_until;
use nom::InputTake;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::parser::parse_name::SimpleNameToken;

pub fn parse_arguments<'a>(s: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, (), Vec<EvaluableToken>> {
    let mut s = s;
    let mut args = Vec::new();
    let mut last = false;

    loop {
        let (ns, section) = if let Ok((ns, section)) = take_until::<_, _, ErrorTree>(",")(s) {
            (ns.take_split(1).0, section)
        }
        else {
            last = true;
            s.take_split(s.len())
        };

        args.push(parse_evaluable(section, containing_class, false)?.1);

        s = ns;
        if last {
            break;
        }
    }

    Ok(((), args))
}