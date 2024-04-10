use crate::root::nom_parser::parse::Location;
use crate::root::nom_parser::parse_function::parse_assigner::AssignmentOperatorToken;
use crate::root::nom_parser::parse_function::parse_evaluable::EvaluableToken;

#[derive(Debug)]
pub struct AssignmentToken {
    location: Location,
    name: String,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken,
}
