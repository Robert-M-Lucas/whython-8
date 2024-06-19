use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use derive_getters::Getters;
use nom::Err::Error;
use nom::Parser;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::tag::TagError;

#[derive(PartialEq, Debug)]
pub enum PrefixOrInfix {
    Prefix,
    Infix,
    Both
}

#[derive(Copy, Clone, Debug)]
pub enum PrefixOrInfixEx {
    Prefix,
    Infix
}

#[derive(Debug, Clone, Getters)]
pub struct OperatorToken {
    location: Location,
    operator: OperatorTokens,
}

const OPERATOR_MAPS: [(&str, OperatorTokens, PrefixOrInfix, &str); 14] = [
    ("+=", OperatorTokens::AsAdd, PrefixOrInfix::Infix, "as_add"),
    ("-=", OperatorTokens::AsSub, PrefixOrInfix::Infix, "as_sub"),
    ("*=", OperatorTokens::AsMul, PrefixOrInfix::Infix, "as_mul"),
    ("/=", OperatorTokens::AsDiv, PrefixOrInfix::Infix, "as_div"),
    ("%=", OperatorTokens::AsMod, PrefixOrInfix::Infix, "as_mod"),
    ("&", OperatorTokens::Reference, PrefixOrInfix::Prefix, "ref"),
    ("+", OperatorTokens::Add, PrefixOrInfix::Both, "add"),
    ("-", OperatorTokens::Subtract, PrefixOrInfix::Both, "sub"),
    ("*", OperatorTokens::Multiply, PrefixOrInfix::Both, "mul"),
    ("/", OperatorTokens::Divide, PrefixOrInfix::Both, "div"),
    ("%", OperatorTokens::Modulo, PrefixOrInfix::Both, "mod"),
    ("==", OperatorTokens::Equals, PrefixOrInfix::Infix, "eq"),
    ("=", OperatorTokens::Assign, PrefixOrInfix::Infix, "assign"),
    ("!", OperatorTokens::Not, PrefixOrInfix::Prefix, "not"),
];

impl OperatorToken {
    pub fn is_prefix_opt_t(&self) -> bool {
        self.operator.is_prefix_op()
    }

    pub fn get_priority_t(&self) -> usize {
        self.operator.get_priority()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OperatorTokens {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Equals,
    AsAdd,
    AsSub,
    AsMul,
    AsDiv,
    AsMod,
    Reference,
    Assign
}

impl OperatorTokens {
    // TODO: Implement functionally
    pub fn is_prefix_op(&self) -> bool {
        for (_, op, prefix, _) in &OPERATOR_MAPS {
            if self == op {
                return match prefix {
                    PrefixOrInfix::Prefix => true,
                    PrefixOrInfix::Infix => false,
                    PrefixOrInfix::Both => true
                }
            }
        }
        panic!()
    }

    pub fn is_infix_op(&self) -> bool {
        for (_, op, prefix, _) in &OPERATOR_MAPS {
            if self == op {
                return match prefix {
                    PrefixOrInfix::Prefix => false,
                    PrefixOrInfix::Infix => true,
                    PrefixOrInfix::Both => true
                }
            }
        }
        panic!()
    }

    pub fn get_method_name(&self, kind: PrefixOrInfixEx) -> Option<String> {
        for (_, op, p_kind, name) in &OPERATOR_MAPS {
            if self == op {
                return match p_kind {
                    PrefixOrInfix::Prefix => {
                        match kind {
                            PrefixOrInfixEx::Prefix => Some(format!("p_{name}")),
                            PrefixOrInfixEx::Infix => None
                        }
                    }
                    PrefixOrInfix::Infix => {
                        match kind {
                            PrefixOrInfixEx::Prefix => None,
                            PrefixOrInfixEx::Infix => Some(name.to_string())
                        }
                    }
                    PrefixOrInfix::Both => {
                        match kind {
                            PrefixOrInfixEx::Prefix => Some(format!("p_{name}")),
                            PrefixOrInfixEx::Infix => Some(name.to_string())
                        }
                    }
                };
            }
        }
        None
    }

    pub fn get_priority(&self) -> usize {
        for (p, (_, op, _, _)) in OPERATOR_MAPS.iter().enumerate() {
            if self == op {
                return p;
            }
        }
        panic!()
    }

    pub fn to_str(&self) -> &'static str {
        for (s, op, _, _) in &OPERATOR_MAPS {
            if self == op {
                return s;
            }
        }
        panic!()
    }
}

pub fn parse_operator(s: Span) -> ParseResult<Span, OperatorToken> {
    for (operator, token, _, _) in OPERATOR_MAPS {
        if let Ok((s, x)) = tag::<_, _, ErrorTree>(operator)(s) {
            return Ok((
                s,
                OperatorToken {
                    location: Location::from_span(&x),
                    operator: token,
                },
            ));
        }
    }

    Err(Error(GenericErrorTree::Alt(
        OPERATOR_MAPS
            .iter()
            .map(|(t, _, _, _)| GenericErrorTree::from_tag(s, *t))
            .collect(),
    )))
}
