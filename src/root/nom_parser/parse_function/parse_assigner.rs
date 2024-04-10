use crate::root::nom_parser::parse::Location;
use crate::root::nom_parser::parse_function::parse_evaluable::EvaluableToken;
use crate::root::nom_parser::parse_function::parse_operator::OperatorTokens;

#[derive(Debug)]
struct AssignmentOperatorToken {
    location: Location,
    assignment_operator: AssignmentOperatorTokens,
}

#[derive(Debug)]
enum AssignmentOperatorTokens {
    None,
    Combination(OperatorTokens),
}

#[derive(Debug)]
pub struct AssignmentToken {
    location: Location,
    name: String,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken,
}
