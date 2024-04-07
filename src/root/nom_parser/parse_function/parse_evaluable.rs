use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::FunctionToken;
use crate::root::nom_parser::parse_name::NameToken;
use crate::root::nom_parser::parse_parameters::Parameters;

#[derive(Debug)]
pub struct EvaluableToken {
    location: Location,
    tokens: EvaluableTokens
}

#[derive(Debug)]
enum EvaluableTokens {
    Name(NameToken),
    Literal(LiteralTokens),
    FunctionCall(NameToken, Parameters),
    InfixOperator(Box<EvaluableToken>, OperatorToken, Box<EvaluableToken>),
    PrefixOperator(OperatorToken, Box<EvaluableToken>)
}

#[derive(Debug)]
struct OperatorToken {
    location: Location,
    operator: OperatorTokens
}

#[derive(Debug)]
pub enum OperatorTokens {
    Add,
    Subtract,
}

#[derive(Debug)]
enum LiteralTokens {
    Bool(bool),
    String(String)
}

pub fn parse_evaluable(s: Span, semicolon_terminated: bool) -> ParseResult<Span, EvaluableToken> {


    todo!()
}
