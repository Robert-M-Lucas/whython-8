use crate::root::basic_ast::symbol::BasicSymbol;
use crate::root::compiler::local_variable::TypeInfo;
use crate::root::custom::types::bool::Bool;
use crate::root::custom::types::float::Float;
use crate::root::custom::types::int::Int;
use crate::root::parser::line_info::LineInfo;
use crate::root::name_resolver::processor::ProcessorError;
use crate::root::name_resolver::type_builder::TypeTable;

#[derive(Clone, strum_macros::Display, Debug)]
#[allow(dead_code)]
pub enum Literal {
    String(String),
    Char(char),
    Int(i128),
    Bool(bool),
    Float(f64),
    Initialiser(String, Vec<Vec<(BasicSymbol, LineInfo)>>),
    Null,
    None,
}

impl Literal {
    pub fn get_type_id(&self, type_table: &TypeTable, line_info: &LineInfo) -> Result<TypeInfo, ProcessorError> {
        Ok(match &self {
            Literal::Int(_) => TypeInfo::new(Int::get_id(), 0),
            Literal::Bool(_) => TypeInfo::new(Bool::get_id(), 0),
            Literal::Float(_) => TypeInfo::new(Float::get_id(), 0),
            Literal::Null => TypeInfo::new(-1, 1),
            Literal::Initialiser(name, _) => TypeInfo::new(
                type_table.get_id_by_name(name).ok_or_else(||
                    ProcessorError::TypeNotFound(line_info.clone(), name.clone())
                )?, 
                0),
            _ => todo!(),
        })
    }
}
