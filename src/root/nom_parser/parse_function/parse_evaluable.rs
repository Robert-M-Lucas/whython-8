use crate::root::nom_parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::nom_parser::parse_function::parse_literal::{
    parse_literal, LiteralToken, LiteralTokens,
};
use crate::root::nom_parser::parse_function::parse_operator::{parse_operator, OperatorToken};
use crate::root::nom_parser::parse_name::{parse_full_name, NameToken};
use b_box::b;
use nom::branch::alt;
use nom::character::complete::{char};
use crate::root::nom_parser::parse_blocks::default_section;
use crate::root::nom_parser::parse_util::discard_ignored;

#[derive(Debug)]
pub struct EvaluableToken {
    location: Location,
    token: EvaluableTokens,
}

pub fn temp_from_token(s: Span, token: EvaluableTokens) -> TempEvaluableTokens {
    TempEvaluableTokens::EvaluableToken(EvaluableToken {
        location: Location::from_span(&s),
        token,
    })
}

#[derive(Debug)]
enum EvaluableTokens {
    Name(NameToken),
    Literal(LiteralToken),
    InfixOperator(Box<EvaluableToken>, OperatorToken, Box<EvaluableToken>),
    PrefixOperator(OperatorToken, Box<EvaluableToken>),
}

#[derive(Debug)]
enum TempEvaluableTokens {
    EvaluableToken(EvaluableToken),
    Operator(OperatorToken),
}

pub fn parse_evaluable(s: Span, semicolon_terminated: bool) -> ParseResult<Span, EvaluableToken> {
    assert!(!s.is_empty());
    let mut s = s;

    let mut evaluables = Vec::new();

    loop {
        let (ns, _) = discard_ignored(s)?;

        if semicolon_terminated {
            if let Ok((ns, _)) = char::<_, ErrorTree>(';')(ns) {
                s = ns;
                break;
            }
        }

        if ns.is_empty() {
            // Fail if semicolon required but ns is empty
            if semicolon_terminated {
                // ! Intentional failure
                char(';')(ns)?;
                unreachable!();
            }

            s = ns;
            break;
        }

        let ns = if let Ok((ns, _)) = default_section(s, '(') {
            let (ns, evaluable) = parse_evaluable(ns, false)?;
            evaluables.push(TempEvaluableTokens::EvaluableToken(evaluable));
            ns
        } else {
            let (ns, token) = alt((
                |x| {
                    parse_literal(x)
                        .map(|(s, t)| (s, temp_from_token(s, EvaluableTokens::Literal(t))))
                },
                |x| parse_operator(x).map(|(s, t)| (s, TempEvaluableTokens::Operator(t))),
                |x| {
                    parse_full_name(x)
                        .map(|(s, t)| (s, temp_from_token(s, EvaluableTokens::Name(t))))
                },
            ))(ns)?;
            evaluables.push(token);
            ns
        };

        s = ns;
    }

    #[derive(Debug)]
    enum TempOperation {
        Infix(Box<TempOperation>, OperatorToken, Box<TempOperation>),
        Prefix(OperatorToken, Box<TempOperation>),
        Value(usize),
    }

    fn parse_prefix(
        section: &[(usize, TempEvaluableTokens)],
    ) -> ParseResult<&[(usize, TempEvaluableTokens)], TempOperation> {
        let TempEvaluableTokens::Operator(operator) = &section[0].1 else {
            panic!()
        };

        if section.len() < 2 {
            todo!()
        }

        let (remaining, operand) = match &section[1] {
            (p, TempEvaluableTokens::EvaluableToken(_)) => {
                (&section[2..], TempOperation::Value(*p))
            }
            (_, TempEvaluableTokens::Operator(_)) => parse_prefix(&section[1..])?,
        };

        Ok((
            remaining,
            TempOperation::Prefix(operator.clone(), Box::new(operand)),
        ))
    }

    let enumerated: Vec<(usize, TempEvaluableTokens)> =
        evaluables.into_iter().enumerate().collect();

    let mut base = None;
    let mut after = Vec::new();

    let mut enumerated_slice: &[(usize, TempEvaluableTokens)] = &enumerated;

    let mut operator_priority = Vec::new();

    while enumerated_slice.len() > 0 {
        let operator = if base.is_some() {
            match &enumerated_slice[0] {
                (_, TempEvaluableTokens::Operator(op)) => {
                    operator_priority.push(op.get_priority_t());
                    enumerated_slice = &enumerated_slice[1..];
                    Some(op.clone())
                }
                (_, TempEvaluableTokens::EvaluableToken(_)) => {
                    // ? Expected infix connecting operator
                    todo!()
                }
            }
        } else {
            None
        };

        let value = match &enumerated_slice[0] {
            (p, TempEvaluableTokens::EvaluableToken(_)) => {
                enumerated_slice = &enumerated_slice[1..];
                TempOperation::Value(*p)
            }
            (_, TempEvaluableTokens::Operator(_)) => {
                let (new_slice, value) = match parse_prefix(enumerated_slice) {
                    Ok(r) => r,
                    Err(_) => {
                        todo!()
                    }
                };
                enumerated_slice = new_slice;
                value
            }
        };

        if base.is_none() {
            base = Some(value);
        } else {
            after.push(Some((operator.unwrap(), value)));
        }
    }

    operator_priority.sort();

    for priority in operator_priority {
        for (pos, (op, _)) in after.iter().map(|x| x.as_ref().unwrap()).enumerate() {
            if op.get_priority_t() != priority {
                continue;
            }

            if pos == 0 {
                let (op, rhs) = after.remove(pos).unwrap();
                base = Some(TempOperation::Infix(b!(base.unwrap()), op, b!(rhs)))
            } else {
                let (op, rhs) = after.remove(pos).unwrap();
                let (lop, base) = after[pos - 1].take().unwrap();
                after[pos - 1] = Some((lop, TempOperation::Infix(b!(base), op, b!(rhs))));
            }

            break;
        }
    }

    debug_assert!(after.is_empty());

    let mut evaluables: Vec<_> = enumerated.into_iter().map(|(_, e)| Some(e)).collect();

    fn recursively_convert_temp(
        base: TempOperation,
        evaluables: &mut Vec<Option<TempEvaluableTokens>>,
    ) -> EvaluableToken {
        fn not_operator(te: TempEvaluableTokens) -> EvaluableToken {
            match te {
                TempEvaluableTokens::EvaluableToken(e) => e,
                TempEvaluableTokens::Operator(_) => {
                    panic!()
                }
            }
        }

        match base {
            TempOperation::Infix(lhs, op, rhs) => {
                let lhs = recursively_convert_temp(*lhs, evaluables);
                EvaluableToken {
                    location: lhs.location.clone(),
                    token: EvaluableTokens::InfixOperator(
                        b!(lhs),
                        op,
                        b!(recursively_convert_temp(*rhs, evaluables)),
                    ),
                }
            }
            TempOperation::Prefix(op, operand) => EvaluableToken {
                location: op.location().clone(),
                token: EvaluableTokens::PrefixOperator(
                    op,
                    b!(recursively_convert_temp(*operand, evaluables)),
                ),
            },
            TempOperation::Value(p) => not_operator(evaluables[p].take().unwrap()),
        }
    }

    Ok((s, recursively_convert_temp(base.unwrap(), &mut evaluables)))
}
