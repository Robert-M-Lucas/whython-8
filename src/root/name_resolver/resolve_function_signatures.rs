use derive_getters::Getters;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult, NameResultId};
use crate::root::shared::common::TypeRef;
use crate::root::parser::parse_function::FunctionToken;

#[derive(Getters)]
pub struct FunctionSignature {
    args: Vec<(String, TypeRef)>,
    return_type: Option<TypeRef>
}

pub fn resolve_function_signature(function_token: &FunctionToken, global_table: &GlobalDefinitionTable) -> Result<FunctionSignature, WError> {
    let mut args = Vec::new();

    let return_type = if let Some((type_name, location)) = function_token.return_type() {
        // TODO
        Some(match global_table.resolve_global_name_to_id(type_name, location)? {
            NameResultId::Function(_) => todo!(),
            NameResultId::Type(type_id) => type_id,
            NameResultId::NotFound => todo!()
        })
    } else {
        None
    };

    for (arg_name, (arg_type, arg_type_loc)) in function_token.parameters() {
        args.push((
            arg_name.0.clone(),
            // TODO
            match global_table.resolve_global_name_to_id(arg_type, arg_type_loc)? {
                NameResultId::Function(_) => todo!(),
                NameResultId::Type(type_ref) => type_ref,
                NameResultId::NotFound => todo!()
            }
        ))
    }

    Ok(FunctionSignature {
        args,
        return_type
    })
}
