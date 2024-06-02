use nom::bytes::complete::{take_until, take_until1};
use nom::InputTake;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};

pub fn parse_arguments<'a, 'b>(s: Span<'a>, containing_class: Option<&'b str>) -> ParseResult<'a, Span<'a>, Vec<EvaluableToken>> {
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

    Ok((s, args))
}