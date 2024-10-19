use crate::root::parser::location::Location;
use crate::root::parser::parse::{ErrorTree, ParseResult, Span};
use derive_getters::Getters;
use nom::Err::Error;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::tag::TagError;

/// Represents whether an operator can be prefix, infix, or both
#[derive(PartialEq, Debug)]
pub enum PrefixOrInfix {
    Prefix,
    Infix,
    Both,
}

/// Represents whether a function is prefix or infix, not both
#[derive(Copy, Clone, Debug)]
pub enum PrefixOrInfixEx {
    Prefix,
    Infix,
}

/// Maps operators to their relevant tokens, whether they are prefix and/or infix,
/// and their function names
const OPERATOR_MAPS: [(&str, OperatorTokens, PrefixOrInfix, &str); 23] = [
    ("+=", OperatorTokens::AsAdd, PrefixOrInfix::Infix, "as_add"),
    ("-=", OperatorTokens::AsSub, PrefixOrInfix::Infix, "as_sub"),
    ("*=", OperatorTokens::AsMul, PrefixOrInfix::Infix, "as_mul"),
    ("/=", OperatorTokens::AsDiv, PrefixOrInfix::Infix, "as_div"),
    ("%=", OperatorTokens::AsMod, PrefixOrInfix::Infix, "as_mod"),
    ("&&", OperatorTokens::And, PrefixOrInfix::Infix, "and"),
    ("&=", OperatorTokens::AsAnd, PrefixOrInfix::Infix, "as_and"),
    ("&", OperatorTokens::Reference, PrefixOrInfix::Prefix, "ref"),
    ("||", OperatorTokens::Or, PrefixOrInfix::Infix, "or"),
    ("|=", OperatorTokens::AsOr, PrefixOrInfix::Infix, "as_or"),
    (
        ">=",
        OperatorTokens::GreaterEqual,
        PrefixOrInfix::Infix,
        "ge",
    ),
    ("<=", OperatorTokens::LessEqual, PrefixOrInfix::Infix, "le"),
    (">", OperatorTokens::GreaterThan, PrefixOrInfix::Infix, "gt"),
    ("<", OperatorTokens::LessThan, PrefixOrInfix::Infix, "lt"),
    ("+", OperatorTokens::Add, PrefixOrInfix::Both, "add"),
    ("-", OperatorTokens::Subtract, PrefixOrInfix::Both, "sub"),
    ("*", OperatorTokens::Multiply, PrefixOrInfix::Both, "mul"),
    ("/", OperatorTokens::Divide, PrefixOrInfix::Both, "div"),
    ("%", OperatorTokens::Modulo, PrefixOrInfix::Both, "mod"),
    ("==", OperatorTokens::Equals, PrefixOrInfix::Infix, "eq"),
    ("!=", OperatorTokens::NotEqual, PrefixOrInfix::Infix, "ne"),
    ("=", OperatorTokens::Assign, PrefixOrInfix::Infix, "assign"),
    ("!", OperatorTokens::Not, PrefixOrInfix::Prefix, "not"),
];

/// A token representing an operator with a location
#[derive(Debug, Clone, Getters)]
pub struct OperatorToken {
    location: Location,
    operator: OperatorTokens,
}

impl OperatorToken {
    pub fn is_prefix(&self) -> bool {
        self.operator.is_prefix()
    }

    pub fn get_priority(&self) -> usize {
        self.operator.get_priority()
    }
}

/// A token representing an operator
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum OperatorTokens {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Not,
    Equals,
    NotEqual,
    And,
    Or,
    GreaterEqual,
    LessEqual,
    GreaterThan,
    LessThan,
    AsAdd,
    AsSub,
    AsMul,
    AsDiv,
    AsMod,
    AsAnd,
    AsOr,
    Reference,
    Assign,
}

impl OperatorTokens {
    pub fn is_prefix(&self) -> bool {
        for (_, op, prefix, _) in &OPERATOR_MAPS {
            if self == op {
                return match prefix {
                    PrefixOrInfix::Prefix => true,
                    PrefixOrInfix::Infix => false,
                    PrefixOrInfix::Both => true,
                };
            }
        }
        panic!()
    }

    pub fn is_infix(&self) -> bool {
        for (_, op, prefix, _) in &OPERATOR_MAPS {
            if self == op {
                return match prefix {
                    PrefixOrInfix::Prefix => false,
                    PrefixOrInfix::Infix => true,
                    PrefixOrInfix::Both => true,
                };
            }
        }
        panic!()
    }

    /// Gets the relevant function name for an operator (changes based on whether it is prefix
    /// or infix)
    pub fn get_method_name(&self, kind: PrefixOrInfixEx) -> Option<String> {
        for (_, op, p_kind, name) in &OPERATOR_MAPS {
            if self == op {
                return match p_kind {
                    PrefixOrInfix::Prefix => match kind {
                        PrefixOrInfixEx::Prefix => Some(format!("p_{name}")),
                        PrefixOrInfixEx::Infix => None,
                    },
                    PrefixOrInfix::Infix => match kind {
                        PrefixOrInfixEx::Prefix => None,
                        PrefixOrInfixEx::Infix => Some(name.to_string()),
                    },
                    PrefixOrInfix::Both => match kind {
                        PrefixOrInfixEx::Prefix => Some(format!("p_{name}")),
                        PrefixOrInfixEx::Infix => Some(name.to_string()),
                    },
                };
            }
        }
        None
    }

    /// Gets the priority value of an operator
    pub fn get_priority(&self) -> usize {
        for (p, (_, op, _, _)) in OPERATOR_MAPS.iter().enumerate() {
            if self == op {
                return p;
            }
        }
        panic!()
    }

    /// Gets the in-code representation of an operator
    pub fn to_str(&self) -> &'static str {
        for (s, op, _, _) in &OPERATOR_MAPS {
            if self == op {
                return s;
            }
        }
        panic!()
    }
}

/// Parses text into an operator
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
