use crate::root::ast::keywords::Keyword;
use crate::root::ast::literals::Literal;
use crate::root::ast::operators::Operator;
use crate::root::basic_ast::punctuation::Punctuation;
use crate::root::parser::line_info::LineInfo;

pub type BasicAbstractSyntaxTree = Vec<(BasicSymbol, LineInfo)>;

#[derive(PartialEq, Clone, strum_macros::Display, Debug)]
pub enum NameAccessType {
    Base,
    Static,
    Normal,
}

#[derive(Clone, strum_macros::Display, Debug)]
pub enum NameType {
    Normal,
    Function(Vec<Vec<(BasicSymbol, LineInfo)>>),
}

#[derive(Clone, strum_macros::Display, Debug)]
pub enum BasicSymbol {
    AbstractSyntaxTree(Vec<(BasicSymbol, LineInfo)>),
    Literal(Literal),
    Operator(Operator),
    Assigner(Option<Operator>),
    BracedSection(Vec<(BasicSymbol, LineInfo)>),
    BracketedSection(Vec<(BasicSymbol, LineInfo)>),
    SquareBracketedSection(Vec<(BasicSymbol, LineInfo)>),
    Punctuation(Punctuation),
    Keyword(Keyword),
    Name(Vec<(String, NameAccessType, NameType, usize)>),
}

pub const NAME_VALID_CHARS: [char; 63] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '0',
];

impl BasicSymbol {
    pub fn get_name_contents(&self) -> &Vec<(String, NameAccessType, NameType, usize)> {
        match self {
            BasicSymbol::Name(inside) => inside,
            _ => panic!(),
        }
    }
}

impl BasicSymbol {
    #[allow(dead_code)]
    pub fn instead_found(&self) -> String {
        match &self {
            BasicSymbol::AbstractSyntaxTree(_) => panic!(),
            BasicSymbol::Literal(_literal) => "Literal (or initialiser)".to_string(),
            BasicSymbol::Operator(_) => "Operator".to_string(),
            BasicSymbol::Assigner(_) => "Assigner".to_string(),
            BasicSymbol::BracedSection(_) => "BracedSection".to_string(),
            BasicSymbol::BracketedSection(_) => "BracketedSection".to_string(),
            BasicSymbol::SquareBracketedSection(_) => "SquareBracketedSection".to_string(),
            BasicSymbol::Punctuation(punctuation) => {
                format!("{punctuation}")
            }
            BasicSymbol::Name(_) => "Name".to_string(),
            BasicSymbol::Keyword(_) => "Keyword".to_string(),
        }
    }
}
