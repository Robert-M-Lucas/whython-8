use nom::character::complete::multispace0;
use crate::root::ast::operators::Operator;
use crate::root::nom_parser::parse::{Location, ParseResult, Span};
use crate::root::nom_parser::parse_blocks::braced_section;
use crate::root::nom_parser::parse_function::FunctionToken;
use crate::root::nom_parser::parse_function::parse_operator::OperatorToken;
use crate::root::nom_parser::parse_name::NameToken;
use crate::root::nom_parser::parse_parameters::Parameters;

#[derive(Debug)]
pub struct EvaluableToken {
    location: Location,
    tokens: EvaluableTokens,
}

#[derive(Debug)]
enum EvaluableTokens {
    Name(NameToken),
    Literal(LiteralTokens),
    FunctionCall(NameToken, Parameters),
    InfixOperator(Box<EvaluableToken>, OperatorToken, Box<EvaluableToken>),
    PrefixOperator(OperatorToken, Box<EvaluableToken>),
}



#[derive(Debug)]
enum LiteralTokens {
    Bool(bool),
    String(String),
}

#[derive(Debug)]
enum TempEvaluableTokens {
    EvaluableToken(EvaluableToken),
    Operator(Operator)
}

pub fn parse_evaluable(s: Span, semicolon_terminated: bool) -> ParseResult<Span, EvaluableToken> {
    let mut s = s;

    let mut evaluables = Vec::new();

    loop {
        let (ns, _) = multispace0(s)?;

        let ns = if let Ok((ns, _)) = braced_section(s) {
            let (ns, evaluable) = parse_evaluable(ns, false)?;
            evaluables.push(TempEvaluableTokens::EvaluableToken(evaluable));
            ns
        }
        else {
            todo!()
        };

        s = ns;
    }

    todo!()
}
