use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::parser::parse_blocks::{
    parse_default_terminator_content, BRACE_TERMINATOR, BRACKET_TERMINATOR,
};
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_function::parse_line::{parse_lines, LineTestFn, LineTokens};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::{discard_ignored, require_ignored};
use derive_getters::Getters;
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;

/// Token holding an if statement
#[derive(Debug, Getters)]
pub struct IfToken {
    location: Location,
    if_condition: EvaluableToken,
    if_contents: Vec<LineTokens>,
    elif_condition_contents: Vec<(EvaluableToken, Vec<LineTokens>)>,
    else_contents: Option<Vec<LineTokens>>,
}

/// Checks if the line should be parsed as an if statement
pub fn test_parse_if<'b>(s: Span<'_>) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match tag("if")(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_if(x, c).map(|(s, x)| (s, LineTokens::If(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parses an if statement
pub fn parse_if<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, IfToken> {
    let (s, l) = tag("if")(s)?; // If
    let (s, _) = discard_ignored(s)?;
    
    // Parse condition
    let (s, content) = parse_default_terminator_content(s, &BRACKET_TERMINATOR)?;
    let (_, if_condition) = parse_evaluable(content, containing_class, false)?;
    let (s, _) = discard_ignored(s)?;
    
    // Parse content
    let (s, contents) = parse_default_terminator_content(s, &BRACE_TERMINATOR)?;
    let (_, if_contents) = parse_lines(contents, containing_class)?;

    // Parse elifs
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

        let (ns, condition) = if let Ok((ns, _)) = (require_ignored, tag("if")).parse(ns) {
            let (ns, _) = discard_ignored(ns)?;
            let (ns, content) = parse_default_terminator_content(ns, &BRACKET_TERMINATOR)?;
            let (_, condition) = parse_evaluable(content, containing_class, false)?;
            (ns, Some(condition))
        } else {
            (ns, None)
        };

        let (ns, _) = discard_ignored(ns)?;

        let (ns, contents) = parse_default_terminator_content(ns, &BRACE_TERMINATOR)?;
        let (_, contents) = parse_lines(contents, containing_class)?;

        // ? Handle else if
        if let Some(condition) = condition {
            elifs.push((condition, contents));
        } else {
            // ? Handle else
            return Ok((
                ns,
                IfToken {
                    location: Location::from_span(&l),
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
    Ok((
        s,
        IfToken {
            location: Location::from_span(&l),
            if_condition,
            if_contents,
            elif_condition_contents: elifs,
            else_contents: None,
        },
    ))
}
