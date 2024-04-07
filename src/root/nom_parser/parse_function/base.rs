use crate::root::nom_parser::parse::Location;
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, OperatorTokens};
use crate::root::nom_parser::parse_function::parse_line::LineTokens;

#[derive(Debug)]
pub struct InitialisationToken {
    location: Location,
    name: String,
    type_name: String,
    value: EvaluableToken
}

#[derive(Debug)]
struct AssignmentOperatorToken {
    location: Location,
    assignment_operator: AssignmentOperatorTokens
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
    value: EvaluableToken
}

#[derive(Debug)]
pub struct IfToken {
    location: Location,
    if_condition: EvaluableToken,
    if_contents: Vec<LineTokens>,
    elif_condition_contents: Vec<(EvaluableToken, Vec<LineTokens>)>,
    else_contents: Option<Vec<LineTokens>>
}

#[derive(Debug)]
pub struct WhileToken {
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens>
}