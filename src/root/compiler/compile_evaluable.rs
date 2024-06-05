use std::collections::HashSet;
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

/// Will always evaluate into new address
pub fn compile_evaluable(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<(String, AddressedTypeRef), WErr> {

    let et = et.token();

    Ok(match et {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    let target = global_table.add_local_variable_unnamed_base(address.type_ref().clone(), local_variables);
                    (copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref())), target)
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            let address = global_table.add_local_variable_unnamed_base(TypeRef::new(tid.clone(), Indirection(0)), local_variables);
            let t = global_table.get_type(tid);

            (t.instantiate_from_literal(address.local_address(), literal)?, address)
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, fid) = compile_evaluable_function_only(fid, inner, local_variables, global_table, function_calls)?;
            function_calls.insert(fid);
            todo!()
        }
    })
}


/// Will always put result into target
pub fn compile_evaluable_into(
    fid: FunctionID,
    et: &EvaluableToken,
    target: AddressedTypeRef,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<String, WErr> {

    let et = et.token();

    Ok(match et {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    if address.type_ref() != target.type_ref() {
                        todo!() // Mismatched types
                    }
                    copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref()))
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            if target.type_ref().indirection().has_indirection() {
                return Err(WErr::n(EvalErrs::BadIndirection(target.type_ref().indirection().0, 0), literal.location().clone()));
            }
            let t = global_table.get_type(*target.type_ref().type_id());

            t.instantiate_from_literal(target.local_address(), literal)?
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, fid) = compile_evaluable_function_only(fid, inner, local_variables, global_table, function_calls)?;
            function_calls.insert(fid);
            todo!()
        }
    })
}

/// Will try to return a reference i.e. not allocate stack
pub fn compile_evaluable_reference(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<(String, AddressedTypeRef), WErr> {

    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    ("".to_string(), address)
                }
            }
        },
        EvaluableTokens::Literal(_) => compile_evaluable(fid, et, local_variables, global_table, function_calls)?,
        EvaluableTokens::InfixOperator(_, _, _) => compile_evaluable(fid, et, local_variables, global_table, function_calls)?,
        EvaluableTokens::PrefixOperator(_, _) => compile_evaluable(fid, et, local_variables, global_table, function_calls)?,
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(_, _) => compile_evaluable(fid, et, local_variables, global_table, function_calls)?
    })
}

/// Will always return a function pointer (and self)
pub fn compile_evaluable_function_only(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<(Option<AddressedTypeRef>, FunctionID), WErr> {

    let et = et.token();

    todo!()
}

/// Will ignore everything other than type
pub fn compile_evaluable_type_only(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    function_calls: &mut HashSet<FunctionID>
) -> Result<TypeRef, WErr> {

    let et = et.token();

    todo!()
}