use derive_getters::Getters;
use itertools::Itertools;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::shared::common::TypeRef;
use crate::root::parser::parse_function::FunctionToken;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_parameters::SelfType;

#[derive(Getters)]
/// A signature for a function
pub struct FunctionSignature {
    self_type: SelfType,
    args: Vec<(SimpleNameToken, TypeRef)>,
    return_type: Option<TypeRef>
}

impl FunctionSignature {
    pub fn new(dynamic: SelfType, args: Vec<(SimpleNameToken, TypeRef)>, return_type: Option<TypeRef>) -> FunctionSignature {
        FunctionSignature {
            self_type: dynamic,
            args,
            return_type
        }
    }

    /// Creates a signature for a builtin (lacking location information) function
    pub fn new_inline_builtin(dynamic: SelfType, args: &[(&str, TypeRef)], return_type: Option<TypeRef>) -> FunctionSignature {
        FunctionSignature {
            self_type: dynamic,
            args: args.iter().map(|(name, t)| (SimpleNameToken::new_builtin(name.to_string()), t.clone())).collect_vec(),
            return_type
        }
    }
}

/// Converts a `FunctionToken` into a `FunctionSignature`
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
        self_type: *function_token.self_type(),
        args,
        return_type
    })
}
