
use nom::sequence::Tuple;
use nom_supreme::tag::complete::tag;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_assigner::{AssignmentOperatorToken, parse_assigner};
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_initialisation::parse_initialisation;
use crate::root::nom_parser::parse_function::parse_line::{LineTestFn, LineTokens};
use crate::root::nom_parser::parse_name::{NameToken, parse_full_name};
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub struct AssignmentToken {
    location: Location,
    name: NameToken,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken,
}

// TODO: Find good way to implement?
pub fn test_parse_assignment<'a>(s: Span<'a>) -> ParseResult<Span, LineTestFn<'a>> {
    Ok((s, |x| parse_assignment(x).map(|(s, x)| (s, LineTokens::Assignment(x)))))
}

pub fn parse_assignment(s: Span) -> ParseResult<Span, AssignmentToken> {
    let (s, _) = discard_ignored(s)?;
    let location = Location::from_span(&s);
    let (s, n) = parse_full_name(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, a) = parse_assigner(s)?;
    let (s, _) = discard_ignored(s)?;
    let (s, e) = parse_evaluable(s, true)?;
    Ok((s, AssignmentToken {
        location,
        name: n,
        assignment_operator: a,
        value: e,
    }))
}