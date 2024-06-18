use std::any::Any;
use std::collections::HashSet;
use either::{Left, Right};
use itertools::Itertools;
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{OpNoReturn, OpWrongReturnType};
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::PrefixOrInfixEx;
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

fn expect_addr(r: (String, Option<AddressedTypeRef>)) -> Result<(String, AddressedTypeRef), WErr> {
    Ok((r.0, r.1.unwrap())) // TODO
}

/// Will always evaluate into new address
pub fn compile_evaluable(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<(String, Option<AddressedTypeRef>), WErr> {

    let et = et.token();

    Ok(match et {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    let target = global_table.add_local_variable_unnamed_base(address.type_ref().clone(), local_variables);
                    (copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref())), Some(target))
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            let address = global_table.add_local_variable_unnamed_base(TypeRef::new(tid.clone(), Indirection(0)), local_variables);
            let t = global_table.get_type(tid);

            (t.instantiate_from_literal(address.local_address(), literal)?, Some(address))
        }
        EvaluableTokens::InfixOperator(_, _, _) => todo!(),
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let signature = global_table.get_function_signature(ifid);
            let return_into = signature.return_type().clone().and_then(|r| Some(global_table.add_local_variable_unnamed_base(r, local_variables)));

            let mut n_args = if let Some(slf) = slf.as_ref() {
                let mut v = Vec::with_capacity(args.len() + 1);
                v.push(Right(slf));
                v
            }
            else {
                Vec::with_capacity(args.len())
            };

            args.iter().for_each(|a| n_args.push(Left(a)));

            let (code, _) = call_function(fid, ifid, &n_args, return_into.clone(), global_table, local_variables, global_tracker)?;
            (code, return_into)
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
    global_tracker: &mut GlobalTracker
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
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Infix)?;
            let signature = global_table.get_function_signature(op_fn);

            if signature.args().len() != 2 {
                return Err(
                    WErr::n(
                        EvalErrs::InfixOpWrongArgumentCount(
                            op.operator().to_str().to_string(),
                            global_table.get_type(*lhs_type.type_id()).name().to_string(),
                            op.operator().get_method_name(PrefixOrInfixEx::Infix).unwrap(),
                            signature.args().len()
                        ),
                        op.location().clone()
                    )
                );
            }

            match signature.return_type() {
                None => {
                    return Err(WErr::n(OpNoReturn(global_table.get_type_name(target.type_ref())), op.location().clone()))
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return Err(WErr::n(OpWrongReturnType(global_table.get_type_name(target.type_ref()), global_table.get_type_name(rt)), op.location().clone()))
                    }
                }
            }

            let (c, _) = call_function(fid, op_fn, &[Left(lhs), Left(rhs)], Some(target), global_table, local_variables, global_tracker)?;

            c
        },
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Prefix)?;
            let signature = global_table.get_function_signature(op_fn);

            if signature.args().len() != 1 {
                return Err(
                    WErr::n(
                        EvalErrs::InfixOpWrongArgumentCount(
                            op.operator().to_str().to_string(),
                            global_table.get_type(*lhs_type.type_id()).name().to_string(),
                            op.operator().get_method_name(PrefixOrInfixEx::Prefix).unwrap(),
                            signature.args().len()
                        ),
                        op.location().clone()
                    )
                );
            }

            match signature.return_type() {
                None => {
                    return Err(WErr::n(OpNoReturn(global_table.get_type_name(target.type_ref())), op.location().clone()))
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return Err(WErr::n(OpWrongReturnType(global_table.get_type_name(target.type_ref()), global_table.get_type_name(rt)), op.location().clone()))
                    }
                }
            }

            let (c, _) = call_function(fid, op_fn, &[Left(lhs)], Some(target), global_table, local_variables, global_tracker)?;
            c
        },
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let mut n_args = if let Some(slf) = slf.as_ref() {
                let mut v = Vec::with_capacity(args.len() + 1);
                v.push(Right(slf));
                v
            }
            else {
                Vec::with_capacity(args.len())
            };

            args.iter().for_each(|a| n_args.push(Left(a)));

            let (code, _) = call_function(fid, ifid, &n_args, Some(target), global_table, local_variables, global_tracker)?;
            code
        }
    })
}

/// Will try to return a reference i.e. not allocate stack
pub fn compile_evaluable_reference(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<(String, Option<AddressedTypeRef>), WErr> {

    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    (String::new(), Some(address))
                }
            }
        },
        EvaluableTokens::Literal(_) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::InfixOperator(_, _, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::PrefixOperator(_, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(_, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
    })
}

/// Will always return a function pointer (and self)
pub fn compile_evaluable_function_only(
    fid: FunctionID,
    name: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<(Option<AddressedTypeRef>, FunctionID), WErr> {
    Ok(match name.token() {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(fid) => {
                    (None, fid)
                }
                _ => todo!(),
            }
        }
        _ => todo!()
    })
}

/// Will ignore everything other than type
pub fn compile_evaluable_type_only(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<TypeRef, WErr> {

    let et = et.token();

    Ok(match et {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return Err(WErr::n(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone())),
                NameResult::Type(_) => return Err(WErr::n(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone())),
                NameResult::Variable(address) => {
                    address.type_ref().clone()
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            TypeRef::new(tid.clone(), Indirection(0))
        }
        EvaluableTokens::InfixOperator(lhs, op, _) => {
            // if op.is_prefix_opt_t() {
            //     return Err(WErr::n(EvalErrs::FoundPrefixNotInfixOp(op.operator().to_str().to_string()), op.location().clone()));
            // }

            // let (mut code, lhs) = compile_evaluable(fid, lhs, local_variables, global_table, global_tracker)?;
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;
            // code += "\n";
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Infix)?;
            let signature = global_table.get_function_signature(op_fn);
            signature.return_type().as_ref().unwrap().clone()
        },
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return Err(WErr::n(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone())), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let signature = global_table.get_function_signature(ifid);
            let return_type = signature.return_type().clone().unwrap(); // TODO: Check type
            return_type
        }
    })
}

/// Will ignore everything other than type with a target type
pub fn compile_evaluable_type_only_into(
    fid: FunctionID,
    et: &EvaluableToken,
    target: TypeRef,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<bool, WErr> {

    let et = et.token();

    todo!()
}