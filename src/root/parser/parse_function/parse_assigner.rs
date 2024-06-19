
// #[derive(Debug)]
// pub struct AssignmentOperatorToken {
//     location: Location,
//     assignment_operator: AssignmentOperatorTokens,
// }
//
// #[derive(Debug, Clone)]
// enum AssignmentOperatorTokens {
//     Normal,
//     Combination(OperatorTokens),
// }
//
// const ASSIGNERS: [(&str, AssignmentOperatorTokens); 2] = [
//     ("=", AssignmentOperatorTokens::Normal),
//     (
//         "+=",
//         AssignmentOperatorTokens::Combination(OperatorTokens::Add),
//     ),
// ];
//
// pub fn parse_assigner(s: Span) -> ParseResult<Span, AssignmentOperatorToken> {
//     let (ns, a) = alt_many(ASSIGNERS.map(|(t, o)| move |x| tag(t)(x).map(|(s, _)| (s, o.clone()))))
//         .parse(s)?;
//
//     Ok((
//         ns,
//         AssignmentOperatorToken {
//             location: Location::from_span(&s),
//             assignment_operator: a,
//         },
//     ))
// }
