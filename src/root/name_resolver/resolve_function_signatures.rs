use derive_getters::Getters;
use itertools::Itertools;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::shared::common::TypeRef;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::SimpleNameToken;

#[derive(Getters)]
pub struct FunctionSignature {
    dynamic: bool,
    args: Vec<(SimpleNameToken, TypeRef)>,
    return_type: Option<TypeRef>
}

impl FunctionSignature {
    pub fn new_inline_builtin(dynamic: bool, args: &[(&str, TypeRef)], return_type: Option<TypeRef>) -> FunctionSignature {
        FunctionSignature {
            dynamic,
            args: args.iter().map(|(name, t)| (SimpleNameToken::new_builtin(name.to_string()), t.clone())).collect_vec(),
            return_type
        }
    }

    pub fn new_custom(dynamic: bool, args: Vec<(SimpleNameToken, TypeRef)>, return_type: Option<TypeRef>) -> FunctionSignature {
        FunctionSignature {
            dynamic,
            args,
            return_type
        }
    }
}

pub fn resolve_function_signature(function_token: &FunctionToken, global_table: &mut GlobalDefinitionTable) -> Result<FunctionSignature, WErr> {
    let mut args = Vec::new();

    let return_type = if let Some(type_name) = function_token.return_type() {
        Some(global_table.resolve_to_type_ref(type_name)?)
    } else {
        None
    };

    for (arg_name, arg_type) in function_token.parameters() {
        args.push((
            arg_name.clone(),
            global_table.resolve_to_type_ref(arg_type)?
        ))
    }


    Ok(FunctionSignature {
        dynamic: *function_token.dynamic(),
        args,
        return_type
    })
}
