use crate::root::parser::parse::{ErrorTree, Location, ParseResult, Span};
use derive_getters::Getters;
use nom::error::{ErrorKind, ParseError};
use nom::Err::Error;
use nom::Parser;
use nom_supreme::error::GenericErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::tag::TagError;

const OPERATOR_MAPS: [(&str, OperatorTokens, bool, &'static str); 4] = [
    ("+", OperatorTokens::Add, false, "add"),
    ("-", OperatorTokens::Subtract, false, "sub"),
    ("==", OperatorTokens::Equals, false, "eq"),
    ("!", OperatorTokens::Not, true, "not"),
];

#[derive(Debug, Clone, Getters)]
pub struct OperatorToken {
    location: Location,
    operator: OperatorTokens,
}

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
    Not,
    Equals
}

impl OperatorTokens {
    // TODO: Implement functionally
    pub fn is_prefix_op(&self) -> bool {
        for (_, op, prefix, _) in &OPERATOR_MAPS {
            if self == op {
                return *prefix;
            }
        }
        panic!()
    }

    pub fn get_method_name(&self) -> &'static str {
        for (_, op, _, name) in &OPERATOR_MAPS {
            if self == op {
                return *name;
            }
        }
        panic!()
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
                return *s;
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
