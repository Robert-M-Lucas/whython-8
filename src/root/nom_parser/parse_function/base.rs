use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_line::LineTokens;
use crate::root::nom_parser::parse_name::NameToken;

#[derive(Debug)]
pub struct EvaluableToken {
    location: Location,
    tokens: Vec<EvaluableTokens>
}

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

#[derive(Debug)]
enum EvaluableTokens {
    Name(NameToken),
    Literal(LiteralTokens),
    InfixOperator(EvaluableToken, OperatorToken, EvaluableToken),
    PrefixOperator(OperatorToken, EvaluableToken)
}

#[derive(Debug)]
struct OperatorToken {
    location: Location,
    operator: OperatorTokens
}

#[derive(Debug)]
enum OperatorTokens {
    Add,
    Subtract,
}

#[derive(Debug)]
enum LiteralTokens {
    Bool(bool),
    String(String)
}

pub fn evaluable(s: Span) -> ParseResult<(), EvaluableToken> {
    todo!()
}