use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse::{ParseResult, Span};
use crate::root::nom_parser::parse_function::base::{AssignmentToken, IfToken};
use crate::root::nom_parser::parse_function::parse_break::{BreakToken, test_parse_break};
use crate::root::nom_parser::parse_function::parse_evaluable::{EvaluableToken, parse_evaluable};
use crate::root::nom_parser::parse_function::parse_initialisation::{InitialisationToken, test_parse_initialisation};
use crate::root::nom_parser::parse_function::parse_return::{ReturnToken, test_parse_return};
use crate::root::nom_parser::parse_function::parse_while::WhileToken;
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub enum LineTokens {
    Initialisation(InitialisationToken),
    Assignment(AssignmentToken),
    If(IfToken),
    While(WhileToken),
    Return(ReturnToken),
    Break(BreakToken),
    NoOp(EvaluableToken)
}

pub type LineTestFn<'a> = fn(Span<'a>) -> ParseResult<Span<'a>, LineTokens>;

pub fn parse_lines(contents: Span) -> ParseResult<(), Vec<LineTokens>> {
    let mut lines = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c);
        if cs.is_empty() {
            break;
        }
        let (cs, function) = parse_line(cs)?;

        lines.push(function);
        c = cs;
    }

    Ok(((), lines))
}

pub fn parse_line(s: Span) -> ParseResult<Span, LineTokens> {
    if let Ok((_, parser)) = alt((
        test_parse_break,
        test_parse_return,
        test_parse_initialisation,
        )).parse(s) {
        parser(s)
    }
    else { // ? Default case is evaluable
        parse_evaluable(s, true).map(|(s, e)| (s, LineTokens::NoOp(e)))
    }
}
