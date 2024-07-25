use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use crate::root::parser::parse_function::parse_literal::{LiteralToken, parse_literal};
use crate::root::parser::parse_function::parse_operator::{OperatorToken, parse_operator};
use b_box::b;
use derive_getters::Getters;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use crate::root::parser::parse_arguments::parse_arguments;
use crate::root::parser::parse_name::{SimpleNameToken, parse_simple_name};
use crate::root::parser::parse_blocks::default_section;
use crate::root::parser::parse_function::parse_struct_init::{parse_struct_init, StructInitToken};
use crate::root::parser::parse_struct::StructToken;
use crate::root::parser::parse_util::discard_ignored;
use crate::root::shared::common::Indirection;

#[derive(Debug, Getters)]
pub struct EvaluableToken {
    location: Location,
    token: EvaluableTokens,
}

#[allow(private_interfaces)]
pub fn temp_from_token(s: Span, token: EvaluableTokens) -> TempEvaluableTokensOne {
    TempEvaluableTokensOne::EvaluableToken(EvaluableToken {
        location: Location::from_span(&s),
        token,
    })
}

#[derive(Debug)]
pub enum EvaluableTokens {
    Name(SimpleNameToken, Option<SimpleNameToken>),
    StaticAccess(Box<EvaluableToken>, SimpleNameToken),
    DynamicAccess(Box<EvaluableToken>, SimpleNameToken),
    FunctionCall(Box<EvaluableToken>, Vec<EvaluableToken>),
    Literal(LiteralToken),
    StructInitialiser(StructInitToken),
    InfixOperator(Box<EvaluableToken>, OperatorToken, Box<EvaluableToken>),
    PrefixOperator(OperatorToken, Box<EvaluableToken>),
    None
}

#[derive(Debug, Getters)]
pub struct FullNameWithIndirectionToken {
    indirection: Indirection,
    inner: FullNameToken
}

impl FullNameWithIndirectionToken {
    pub fn from_simple(simple: SimpleNameToken, containing_class: Option<SimpleNameToken>, location: Location) -> FullNameWithIndirectionToken {
        FullNameWithIndirectionToken {
            indirection: Indirection(0),
            inner: FullNameToken {
                location,
                token: FullNameTokens::Name(simple, containing_class)
            }
        }
    }

    pub fn into_inner(self) -> FullNameToken {
        self.inner
    }
}

#[derive(Debug, Getters)]
pub struct FullNameToken {
    location: Location,
    token: FullNameTokens
}

impl FullNameToken {
    pub fn new(location: Location, token: FullNameTokens) -> FullNameToken {
        FullNameToken {
            location,
            token,
        }
    }

    pub fn with_no_indirection(self) -> FullNameWithIndirectionToken {
        FullNameWithIndirectionToken {
            indirection: Indirection(0),
            inner: self
        }
    }

    pub fn into_evaluable(self) -> EvaluableToken {
        let (location, token) = (self.location, self.token);
        let token = token.into_evaluable_token();
        EvaluableToken {
            location,
            token,
        }
    }
}

#[derive(Debug)]
pub enum FullNameTokens {
    Name(SimpleNameToken, Option<SimpleNameToken>),
    StaticAccess(Box<FullNameToken>, SimpleNameToken),
    DynamicAccess(Box<FullNameToken>, SimpleNameToken)
}

impl FullNameTokens {
    pub fn into_evaluable_token(self) -> EvaluableTokens {
        match self {
            FullNameTokens::Name(n, c) => EvaluableTokens::Name(n, c),
            FullNameTokens::StaticAccess(e, n) => EvaluableTokens::StaticAccess(b!(e.into_evaluable()), n),
            FullNameTokens::DynamicAccess(e, n) => EvaluableTokens::DynamicAccess(b!(e.into_evaluable()), n),
        }
    }
}

#[allow(private_interfaces)]
#[derive(Debug)]
enum TempEvaluableTokensOne {
    EvaluableToken(EvaluableToken),
    Operator(OperatorToken),
    StaticAccess(SimpleNameToken),
    DynamicAccess(SimpleNameToken),
    FunctionCall(SimpleNameToken, Option<SimpleNameToken>, Vec<EvaluableToken>),
    StaticFunctionCall(SimpleNameToken, Vec<EvaluableToken>),
    DynamicFunctionCall(SimpleNameToken, Vec<EvaluableToken>),
}

#[derive(Debug)]
enum TempEvaluableTokensTwo {
    EvaluableToken(EvaluableToken),
    Operator(OperatorToken),
}

pub fn parse_full_name<'a>(s: Span<'a>, containing_class: Option<&SimpleNameToken>) -> ParseResult<'a, Span<'a>, FullNameWithIndirectionToken> {
    let mut indirection: usize = 0;
    let mut s = s;
    loop {
        let (ns, _) = discard_ignored(s)?;

        if let Ok((ns, _)) = char::<Span, ErrorTree>('&')(ns) {
            indirection += 1;
            s = ns;
        }
        else {
            s = ns;
            break;
        }
    }

    let (s, _) = discard_ignored(s)?;
    let (s, section) = parse_simple_name(s)?;

    let mut current = FullNameToken {
        location: section.location().clone(),
        token: FullNameTokens::Name(section, containing_class.cloned())
    };

    let mut s = s;

    let (ns, _) = discard_ignored(s)?;

    if let Ok((ns, _)) = tag::<&str, Span, ErrorTree>("::")(ns) {
        let (ns, section) = parse_simple_name(ns)?;
        current = FullNameToken {
            location: section.location().clone(),
            token: FullNameTokens::StaticAccess(b!(current), section),
        };
        s = ns;
    }
    if let Ok((ns, _)) = char::<Span, ErrorTree>('.')(ns) {
        let (ns, section) = parse_simple_name(ns)?;
        current = FullNameToken {
            location: section.location().clone(),
            token: FullNameTokens::DynamicAccess(b!(current), section),
        };
        s = ns;
    }

    Ok((s, FullNameWithIndirectionToken { indirection: Indirection(indirection), inner: current }))
}

// pub fn error_on_assignment(either: Either<EvaluableToken, AssignmentToken>) -> Result<EvaluableToken, ErrorTree<'static>> {
//     match either {
//         Left(val) => {Ok(val)}
//         Right(_) => {
//             todo!()
//         }
//     }
// }

pub fn parse_evaluable<'a, 'b>(s: Span<'a>, containing_class: Option<&'b SimpleNameToken>, semicolon_terminated: bool) -> ParseResult<'a, Span<'a>, EvaluableToken> {
    let mut s = s;

    let mut evaluables = Vec::new();

    // Collect evaluable sections into initial vec
    loop {
        let (ns, _) = discard_ignored(s)?;

        // Terminate on semicolon if semicolon terminated
        if semicolon_terminated {
            if let Ok((ns, _)) = char::<_, ErrorTree>(';')(ns) {
                s = ns;
                break;
            }
        }

        // Fail if semicolon terminated but reach end of span
        if ns.is_empty() {
            if semicolon_terminated {
                // ! Intentional failure
                char(';')(ns)?;
                unreachable!();
            }

            if evaluables.is_empty() {
                return Ok((ns, EvaluableToken {
                    location: Location::from_span(&ns),
                    token: EvaluableTokens::None,
                }))
            }

            s = ns;
            break;
        }

        // Recursively parse bracketed sections
        let ns = if let Ok((ns, inner)) = default_section(s, '(') {
            let (_, evaluable) = parse_evaluable(inner, containing_class, false)?;
            evaluables.push(TempEvaluableTokensOne::EvaluableToken(evaluable));
            ns
        }
        // Parse evaluable
        else {
            let (ns, token) = alt((
                |x| {
                    parse_literal(x)
                        .map(|(s, t)| (s, temp_from_token(s, EvaluableTokens::Literal(t))))
                },
                |x| parse_operator(x).map(|(s, t)| (s, TempEvaluableTokensOne::Operator(t))),
                |x| parse_struct_init(x, containing_class.clone()).map(|(s, t)| (s, temp_from_token(s, EvaluableTokens::StructInitialiser(t)))),
                |x| {
                    enum Kind {
                        Static,
                        Dynamic,
                        None
                    }

                    let (x, kind) = tag::<&str, Span, ErrorTree>("::")(x).map(|(a, _)| (a, Kind::Static))
                        .map_err(|_|
                                     char::<Span, ErrorTree>('.')(x).map(|a| (a, Kind::Dynamic)))
                        .unwrap_or((x, Kind::None));

                    let (x, section) = parse_simple_name(x)?;

                    Ok(if char::<Span, ErrorTree>('(')(x).is_ok() {
                        let (x, arguments) = default_section(x, '(')?;
                        let (_, arguments) = parse_arguments(arguments, containing_class)?;
                        (x, match kind {
                            Kind::Static => TempEvaluableTokensOne::StaticFunctionCall(section, arguments),
                            Kind::Dynamic => TempEvaluableTokensOne::DynamicFunctionCall(section, arguments),
                            Kind::None => TempEvaluableTokensOne::FunctionCall(section, containing_class.cloned(), arguments)
                        })
                    }
                    else {
                        match kind {
                            Kind::Static => (x, TempEvaluableTokensOne::StaticAccess(section)),
                            Kind::Dynamic => (x, TempEvaluableTokensOne::DynamicAccess(section)),
                            Kind::None => (x, TempEvaluableTokensOne::EvaluableToken(EvaluableToken {
                                location: section.location().clone(),
                                token: EvaluableTokens::Name(section, containing_class.cloned()),
                            })),
                        }
                    })
                },
            ))(ns)?;
            evaluables.push(token);
            ns
        };

        s = ns;
    }

    let mut new_evaluables: Vec<TempEvaluableTokensTwo> = Vec::new();

    for eval in evaluables {
        match eval {
            TempEvaluableTokensOne::StaticAccess(n) => {
                match new_evaluables.pop() {
                    Some(TempEvaluableTokensTwo::Operator(_)) => todo!(), // Can't be operator
                    Some(TempEvaluableTokensTwo::EvaluableToken(e)) => {
                        new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(EvaluableToken {
                            location: e.location.clone(),
                            token: EvaluableTokens::StaticAccess(b!(e), n),
                        }))
                    },
                    None => todo!(), // Must have previous
                }
            }
            TempEvaluableTokensOne::DynamicAccess(n) => {
                match new_evaluables.pop() {
                    Some(TempEvaluableTokensTwo::Operator(_)) => todo!(), // Can't be operator
                    Some(TempEvaluableTokensTwo::EvaluableToken(e)) => {
                        new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(EvaluableToken {
                            location: e.location.clone(),
                            token: EvaluableTokens::DynamicAccess(b!(e), n),
                        }))
                    },
                    None => todo!(), // Must have previous
                }
            }
            TempEvaluableTokensOne::FunctionCall(n, c, a) => {
                new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(EvaluableToken {
                    location: n.location().clone(),
                    token: EvaluableTokens::FunctionCall(b!(EvaluableToken { location: n.location().clone(), token: EvaluableTokens::Name(n, c) }), a),
                }))
            }
            TempEvaluableTokensOne::DynamicFunctionCall(n, a) => {
                match new_evaluables.pop() {
                    Some(TempEvaluableTokensTwo::Operator(_)) => todo!(), // Can't be operator
                    Some(TempEvaluableTokensTwo::EvaluableToken(e)) => {
                        new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(EvaluableToken {
                            location: e.location.clone(),
                            token: EvaluableTokens::FunctionCall(b!(EvaluableToken { location: n.location().clone(), token: EvaluableTokens::DynamicAccess(b!(e), n) }), a),
                        }))
                    },
                    None => todo!(), // Must have previous
                }
            }
            TempEvaluableTokensOne::StaticFunctionCall(n, a) => {
                match new_evaluables.pop() {
                    Some(TempEvaluableTokensTwo::Operator(_)) => todo!(), // Can't be operator
                    Some(TempEvaluableTokensTwo::EvaluableToken(e)) => {
                        new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(EvaluableToken {
                            location: e.location.clone(),
                            token: EvaluableTokens::FunctionCall(b!(EvaluableToken { location: n.location().clone(), token: EvaluableTokens::StaticAccess(b!(e), n) }), a),
                        }))
                    },
                    None => todo!(), // Must have previous
                }
            }
            TempEvaluableTokensOne::EvaluableToken(e) => new_evaluables.push(TempEvaluableTokensTwo::EvaluableToken(e)),
            TempEvaluableTokensOne::Operator(o) => new_evaluables.push(TempEvaluableTokensTwo::Operator(o)),
        };
    }

    let evaluables = new_evaluables;

    #[derive(Debug)]
    enum TempOperation {
        Infix(Box<TempOperation>, OperatorToken, Box<TempOperation>),
        Prefix(OperatorToken, Box<TempOperation>),
        Value(usize),
    }

    fn parse_prefix(
        section: &[(usize, TempEvaluableTokensTwo)],
    ) -> ParseResult<&[(usize, TempEvaluableTokensTwo)], TempOperation> {
        let TempEvaluableTokensTwo::Operator(operator) = &section[0].1 else {
            panic!()
        };

        if section.len() < 2 {
            todo!()
        }

        let (remaining, operand) = match &section[1] {
            (p, TempEvaluableTokensTwo::EvaluableToken(_)) => {
                (&section[2..], TempOperation::Value(*p))
            }
            (_, TempEvaluableTokensTwo::Operator(_)) => parse_prefix(&section[1..])?,
        };

        Ok((
            remaining,
            TempOperation::Prefix(operator.clone(), Box::new(operand)),
        ))
    }

    let enumerated: Vec<(usize, TempEvaluableTokensTwo)> =
        evaluables.into_iter().enumerate().collect();

    let mut base = None;
    let mut after = Vec::new();

    let mut enumerated_slice: &[(usize, TempEvaluableTokensTwo)] = &enumerated;

    let mut operator_priority = Vec::new();

    while !enumerated_slice.is_empty() {
        let operator = if base.is_some() {
            match &enumerated_slice[0] {
                (_, TempEvaluableTokensTwo::Operator(op)) => {
                    operator_priority.push(op.get_priority_t());
                    enumerated_slice = &enumerated_slice[1..];
                    Some(op.clone())
                }
                (_, TempEvaluableTokensTwo::EvaluableToken(_)) => {
                    // ? Expected infix connecting operator
                    todo!()
                }
            }
        } else {
            None
        };

        let value = match &enumerated_slice[0] {
            (p, TempEvaluableTokensTwo::EvaluableToken(_)) => {
                enumerated_slice = &enumerated_slice[1..];
                TempOperation::Value(*p)
            }
            (_, TempEvaluableTokensTwo::Operator(_)) => {
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
        evaluables: &mut Vec<Option<TempEvaluableTokensTwo>>,
    ) -> EvaluableToken {
        fn not_operator(te: TempEvaluableTokensTwo) -> EvaluableToken {
            match te {
                TempEvaluableTokensTwo::EvaluableToken(e) => e,
                TempEvaluableTokensTwo::Operator(_) => {
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

    let conv = recursively_convert_temp(base.unwrap(), &mut evaluables);

    Ok((s, conv))
}
