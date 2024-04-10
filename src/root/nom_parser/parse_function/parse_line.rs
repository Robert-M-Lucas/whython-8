use crate::root::nom_parser::parse::{ErrorTree, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_assignment::{AssignmentToken, test_parse_assignment};
use crate::root::nom_parser::parse_function::parse_break::{test_parse_break, BreakToken};
use crate::root::nom_parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::nom_parser::parse_function::parse_if::{test_parse_if, IfToken};
use crate::root::nom_parser::parse_function::parse_initialisation::{
    test_parse_initialisation, InitialisationToken,
};
use crate::root::nom_parser::parse_function::parse_return::{test_parse_return, ReturnToken};
use crate::root::nom_parser::parse_function::parse_while::{test_parse_while, WhileToken};
use nom::branch::alt;
use nom::Parser;
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub enum LineTokens {
    Initialisation(InitialisationToken),
    Assignment(AssignmentToken),
    If(IfToken),
    While(WhileToken),
    Return(ReturnToken),
    Break(BreakToken),
    NoOp(EvaluableToken),
}

pub type LineTestFn<'a> = fn(Span<'a>) -> ParseResult<Span<'a>, LineTokens>;

pub fn parse_lines(contents: Span) -> ParseResult<(), Vec<LineTokens>> {
    let mut lines = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c)?;
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
        test_parse_while,
        test_parse_if,
        test_parse_assignment,
    ))
    .parse(s)
    {
        parser(s)
    } else {
        // ? Default case is evaluable
        parse_evaluable(s, true).map(|(s, e)| (s, LineTokens::NoOp(e)))
    }
}
