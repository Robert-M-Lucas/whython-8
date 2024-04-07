use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_name::NameToken;

enum LineTokens<'a> {
    Initialisation(InitialisationToken<'a>),
    Assignment(AssignmentToken<'a>),
    If(IfToken<'a>),
    While(WhileToken<'a>),
    Return(&'a str),
    Break,
    NoOp(EvaluableToken)
}

#[derive(Debug)]
pub struct EvaluableToken {
    location: Location,
    tokens: Vec<EvaluableTokens>
}

struct InitialisationToken<'a> {
    location: Location,
    name: &'a str,
    type_name: &'a str,
    value: EvaluableToken
}

struct AssignmentOperatorToken {
    location: Location,
    assignment_operator: AssignmentOperatorTokens
}

enum AssignmentOperatorTokens {
    None,
    Combination(OperatorTokens),
}

struct AssignmentToken<'a> {
    location: Location,
    name: &'a str,
    assignment_operator: AssignmentOperatorToken,
    value: EvaluableToken
}

struct IfToken<'a> {
    location: Location,
    if_condition: EvaluableToken,
    if_contents: Vec<LineTokens<'a>>,
    elif_condition_contents: Vec<(EvaluableToken, Vec<LineTokens<'a>>)>,
    else_contents: Option<Vec<LineTokens<'a>>>
}

struct WhileToken<'a> {
    location: Location,
    condition: EvaluableToken,
    contents: Vec<LineTokens<'a>>
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