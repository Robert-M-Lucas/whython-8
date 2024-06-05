use derive_getters::Getters;
use crate::root::errors::WError;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::shared::common::TypeRef;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::SimpleNameToken;

#[derive(Getters)]
pub struct FunctionSignature {
    args: Vec<(SimpleNameToken, TypeRef)>,
    return_type: Option<TypeRef>
}

pub fn resolve_function_signature(function_token: &FunctionToken, global_table: &GlobalDefinitionTable) -> Result<FunctionSignature, WError> {
    let mut args = Vec::new();

    let return_type = if let Some((type_name, location)) = function_token.return_type() {
        // TODO
        Some(global_table.resolve_global_name_to_id(type_name)?)
    } else {
        None
    };

    for (arg_name, arg_type) in function_token.parameters() {
        args.push((
            arg_name.clone(),
            global_table.resolve_global_name_to_id(arg_type)?
        ))
    }

    Ok(FunctionSignature {
        args,
        return_type
    })
}
