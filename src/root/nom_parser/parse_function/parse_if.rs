
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

use crate::root::nom_parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::default_section;
use crate::root::nom_parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::nom_parser::parse_function::parse_line::{parse_lines, LineTestFn, LineTokens};
use crate::root::nom_parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug)]
pub struct IfToken {
    location: Location,
    if_condition: EvaluableToken,
    if_contents: Vec<LineTokens>,
    elif_condition_contents: Vec<(EvaluableToken, Vec<LineTokens>)>,
    else_contents: Option<Vec<LineTokens>>,
}

pub fn test_parse_if<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    match tag("if")(s) {
        Ok(_) => Ok((s, |x| parse_if(x).map(|(s, x)| (s, LineTokens::If(x))))),
        Err(e) => Err(e),
    }
}

pub fn parse_if(s: Span) -> ParseResult<Span, IfToken> {
    let (s, l) = tag("if")(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, content) = default_section(s, '(')?;
    let (_, if_condition) = parse_evaluable(content, false)?;
    let (s, _) = discard_ignored(s)?;
    let (s, contents) = default_section(s, '{')?;
    let (_, if_contents) = parse_lines(contents)?;

    let mut elifs = Vec::new();
    let mut s = s;

    loop {
        let (ns, _) = discard_ignored(s)?;

        if ns.is_empty() {
            break;
        }

        let ns = if let Ok((ns, _)) = tag::<_, _, ErrorTree>("else")(ns) {
            ns
        } else {
            s = ns;
            break;
        };

        let (ns, condition) =
            if let Ok((ns, _)) = (require_ignored, tag("if")).parse(ns) {
                let (ns, _) = discard_ignored(ns)?;
                let (ns, content) = default_section(ns, '(')?;
                let (_, condition) = parse_evaluable(content, false)?;
                (ns, Some(condition))
            } else {
                (ns, None)
            };

        let (ns, _) = discard_ignored(ns)?;

        let (ns, contents) = default_section(ns, '{')?;
        let (_, contents) = parse_lines(contents)?;

        // ? Handle else if
        if let Some(condition) = condition {
            elifs.push((condition, contents));
        } else {
            // ? Handle else
            return Ok((
                ns,
                IfToken {
                    location: Location::from_span(l),
                    if_condition,
                    if_contents,
                    elif_condition_contents: elifs,
                    else_contents: Some(contents),
                },
            ));
        }

        s = ns;
    }

    // ? Ended without else
    return Ok((
        s,
        IfToken {
            location: Location::from_span(l),
            if_condition,
            if_contents,
            elif_condition_contents: elifs,
            else_contents: None,
        },
    ));
}
