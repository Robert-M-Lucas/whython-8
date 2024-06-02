use derive_getters::Getters;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult, NameResultId};
use crate::root::name_resolver::resolve::TypeRef;
use crate::root::parser::parse_function::FunctionToken;

#[derive(Getters)]
pub struct FunctionSignature {
    args: Vec<(String, TypeRef)>,
    return_type: Option<TypeRef>
}

pub fn resolve_function_signature(function_token: &FunctionToken, global_table: &GlobalDefinitionTable) -> FunctionSignature {
    let mut args = Vec::new();

    let return_type = if let Some(type_name) = function_token.return_type() {
        Some(match global_table.resolve_global_name_to_id(type_name) {
            NameResultId::Function(_) => todo!(),
            NameResultId::Type(type_id) => TypeRef::new(type_id, *type_name.indirection()),
            NameResultId::NotFound => todo!()
        })
    } else {
        None
    };

    for (arg_name, arg_type) in function_token.parameters() {
        args.push((
            arg_name.clone(),
            match global_table.resolve_global_name_to_id(arg_type) {
                NameResultId::Function(_) => todo!(),
                NameResultId::Type(type_id) => TypeRef::new(type_id, *arg_type.indirection()),
                NameResultId::NotFound => todo!()
            }
        ))
    }

    FunctionSignature {
        args,
        return_type
    }
}
