use crate::root::parser::location::Location;
use crate::root::parser::parse::{ParseResult, Span};
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

/// Token representing a while statement with location
#[derive(Debug, Getters)]
pub struct WhileToken {
    #[allow(dead_code)]
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens>,
}

/// Tests whether a line should be parsed as a while statement
pub fn test_parse_while<'b>(s: Span) -> ParseResult<Span, LineTestFn<'_, 'b>> {
    match (tag("while"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_while(x, c).map(|(s, x)| (s, LineTokens::While(x)))
        })),
        Err(e) => Err(e),
    }
}

/// Parses a while statement
pub fn parse_while<'a>(
    s: Span<'a>,
    containing_class: Option<&SimpleNameToken>,
) -> ParseResult<'a, Span<'a>, WhileToken> {
    let (s, l) = tag("while")(s)?;
    let (s, _) = discard_ignored(s)?;
    // Get condition
    let (s, content) = parse_default_terminator_content(s, &BRACKET_TERMINATOR)?;
    let (_, condition) = parse_evaluable(content, containing_class, false)?;
    let (s, _) = discard_ignored(s)?;
    // Get contents
    let (s, contents) = parse_default_terminator_content(s, &BRACE_TERMINATOR)?;

    let (_, lines) = parse_lines(contents, containing_class)?;

    Ok((
        s,
        WhileToken {
            location: Location::from_span(&l),
            condition,
            contents: lines,
        },
    ))
}
