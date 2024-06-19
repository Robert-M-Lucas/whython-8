use derive_getters::Getters;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_assigner::AssignmentOperatorToken;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, FullNameWithIndirectionToken};

#[derive(Debug, Getters)]
pub struct AssignmentToken {
    location: Location,
    name: FullNameWithIndirectionToken,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken,
}

// TODO: Find good way to implement?
// pub fn test_parse_assignment<'a, 'b>(s: Span<'a>) -> ParseResult<'a, Span<'a>, LineTestFn<'a, 'b>> {
//     let (s, _) = discard_ignored(s)?;
//     let (s, _) = parse_full_name(s, None)?;
//     let (s, _) = discard_ignored(s)?;
//     let (s, _) = parse_assigner(s)?;
//
//     Ok((s, |x, c| parse_assignment(x, c).map(|(s, x)| (s, LineTokens::Assignment(x)))))
// }
//
// pub fn parse_assignment<'a, 'b>(s: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, Span<'a>, AssignmentToken> {
//     let (s, _) = discard_ignored(s)?;
//     let location = Location::from_span(&s);
//     let (s, n) = parse_full_name(s, containing_class)?;
//     let (s, _) = discard_ignored(s)?;
//     let (s, a) = parse_assigner(s)?;
//     let (s, _) = discard_ignored(s)?;
//     let (s, e) = parse_evaluable(s, containing_class, true)?;
//     Ok((s, AssignmentToken {
//         location,
//         name: n,
//         assignment_operator: a,
//         value: e,
//     }))
// }