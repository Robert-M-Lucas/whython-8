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

// TODO: Implement functionally
pub fn is_prefix_op(operator: &OperatorTokens) -> bool {
    for (_, op, prefix, _) in &OPERATOR_MAPS {
        if operator == op {
            return *prefix;
        }
    }
    panic!()
}

pub fn get_method_name(operator: &OperatorTokens) -> &'static str {
    for (_, op, _, name) in &OPERATOR_MAPS {
        if operator == op {
            return *name;
        }
    }
    panic!()
}

pub fn get_priority(operator: &OperatorTokens) -> usize {
    for (p, (_, op, _, _)) in OPERATOR_MAPS.iter().enumerate() {
        if operator == op {
            return p;
        }
    }
    panic!()
}

#[derive(Debug, Clone, Getters)]
pub struct OperatorToken {
    location: Location,
    operator: OperatorTokens,
}

impl OperatorToken {
    pub fn is_prefix_opt_t(&self) -> bool {
        is_prefix_op(&self.operator)
    }

    pub fn get_priority_t(&self) -> usize {
        get_priority(&self.operator)
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
    pub fn is_prefix_opt_t(&self) -> bool {
        is_prefix_op(&self)
    }

    pub fn get_priority_t(&self) -> usize {
        get_priority(&self)
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
