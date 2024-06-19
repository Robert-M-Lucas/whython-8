use crate::root::parser::parse::{ParseResult, Span};
use crate::root::parser::parse_function::parse_break::{test_parse_break, BreakToken};
use crate::root::parser::parse_function::parse_evaluable::{parse_evaluable, EvaluableToken};
use crate::root::parser::parse_function::parse_if::{test_parse_if, IfToken};
use crate::root::parser::parse_function::parse_initialisation::{
    test_parse_initialisation, InitialisationToken,
};
use crate::root::parser::parse_function::parse_return::{test_parse_return, ReturnToken};
use crate::root::parser::parse_function::parse_while::{test_parse_while, WhileToken};
use nom::branch::alt;
use nom::Parser;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_util::discard_ignored;

#[derive(Debug)]
pub enum LineTokens {
    Initialisation(InitialisationToken),
    If(IfToken),
    While(WhileToken),
    Return(ReturnToken),
    Break(BreakToken),
    NoOp(EvaluableToken),
}

/// fn(line span, Option<class name>)
pub type LineTestFn<'a, 'b> = fn(Span<'a>, Option<&'b SimpleNameToken>) -> ParseResult<'a, Span<'a>, LineTokens>;

pub fn parse_lines<'a>(contents: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, (), Vec<LineTokens>> {
    let mut lines = Vec::new();

    let mut c = contents;
    loop {
        let (cs, _) = discard_ignored(c)?;
        if cs.is_empty() {
            break;
        }

        let (cs, function) = parse_line(cs, containing_class)?;

        lines.push(function);
        c = cs;
    }

    Ok(((), lines))
}

pub fn parse_line<'a>(s: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, Span<'a>, LineTokens> {
    if let Ok((_, parser)) = alt((
        test_parse_break,
        test_parse_return,
        test_parse_initialisation,
        test_parse_while,
        test_parse_if,
        // test_parse_assignment,
    ))
    .parse(s)
    {
        parser(s, containing_class)
    } else {
        // ? Default case is evaluable
        parse_evaluable(s, containing_class, true).map(|(s, e)| (s, LineTokens::NoOp(e)))
    }
}
