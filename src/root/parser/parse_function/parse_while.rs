use derive_getters::Getters;
use nom::character::complete::{satisfy};
use nom::sequence::Tuple;
use nom::Parser;
use nom_supreme::tag::complete::tag;

use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken,};
use crate::root::parser::parse_function::parse_line::{parse_lines, LineTestFn, LineTokens};
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::{discard_ignored, require_ignored};

#[derive(Debug, Getters)]
pub struct WhileToken {
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens>,
}

pub fn test_parse_while<'a, 'b>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a, 'b>> {
    match (tag("while"), require_ignored).parse(s) {
        Ok(_) => Ok((s, |x, c| {
            parse_while(x, c).map(|(s, x)| (s, LineTokens::While(x)))
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_while<'a, 'b>(s: Span<'a>, containing_class: Option<&'b SimpleNameToken>) -> ParseResult<'a, Span<'a>, WhileToken> {
    let (s, l) = tag("while")(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, content) = default_section(s, '(')?;
    let (_, condition) = parse_evaluable(content, containing_class, false)?;
    let (s, _) = discard_ignored(s)?;
    let (s, contents) = default_section(s, '{')?;

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
