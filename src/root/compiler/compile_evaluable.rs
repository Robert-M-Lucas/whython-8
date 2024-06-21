use std::any::Any;
use either::{Left, Right};
use itertools::Itertools;
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedDifferentType, ExpectedFunctionName, ExpectedNotNone, OpNoReturn, OpWrongReturnType};
use crate::root::errors::name_resolver_errors::NRErrors;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

fn expect_addr(r: (String, Option<AddressedTypeRef>)) -> Result<(String, AddressedTypeRef), WErr> {
    Ok((r.0, r.1.unwrap())) // TODO
}

pub fn set_reference(location: &Location, to_ref: AddressedTypeRef, into: AddressedTypeRef, global_table: &GlobalDefinitionTable) -> Result<String, WErr> {
    let new_type = to_ref.type_ref().type_id().with_indirection(to_ref.type_ref().indirection().0 + 1);
    if new_type != *into.type_ref() {
        return WErr::ne(OpWrongReturnType(global_table.get_type_name(into.type_ref()), global_table.get_type_name(&new_type)), location.clone());
    }

    Ok(format!("    mov rax, rbp
    add rax, {}
    mov qword {}, rax\n", to_ref.local_address().0, into.local_address()))
}

/// Will always evaluate into new address
pub fn compile_evaluable(
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
                NameResult::Function(_) => return WErr::ne(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone()),
                NameResult::Type(_) => return WErr::ne(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone()),
                NameResult::Variable(address) => {
                    let target = global_table.add_local_variable_unnamed_base(address.type_ref().clone(), local_variables);
                    (copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref())), Some(target))
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            let address = global_table.add_local_variable_unnamed_base(TypeRef::new(tid, Indirection(0)), local_variables);
            let t = global_table.get_type(tid);

            (t.instantiate_from_literal(address.local_address(), literal)?, Some(address))
        }
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Infix)?;
            let signature = global_table.get_function_signature(op_fn);

            if signature.get().args().len() != 2 {
                return WErr::ne(
                    EvalErrs::InfixOpWrongArgumentCount(
                        op.operator().to_str().to_string(),
                        global_table.get_type(*lhs_type.type_id()).name().to_string(),
                        op.operator().get_method_name(PrefixOrInfixEx::Infix).unwrap(),
                        signature.get().args().len()
                    ),
                    op.location().clone()
                );
            }

            let return_type = signature.get().return_type().clone();
            let return_into = return_type.map(|rt| global_table.add_local_variable_unnamed_base(rt.clone(), local_variables));

            let (c, _) = call_function(fid, op_fn, op.location(), &op.operator().get_method_name(PrefixOrInfixEx::Infix).unwrap(), &[Left(lhs), Left(rhs)], return_into.clone(), global_table, local_variables, global_tracker)?;

            (c, return_into)
        },
        EvaluableTokens::PrefixOperator(_, _) => todo!(),
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid, name) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let signature = global_table.get_function_signature(ifid);
            let return_into = signature.get().return_type().clone().map(|r| global_table.add_local_variable_unnamed_base(r, local_variables));

            let mut n_args = if let Some(slf) = slf.as_ref() {
                let mut v = Vec::with_capacity(args.len() + 1);
                v.push(Right(slf));
                v
            }
            else {
                Vec::with_capacity(args.len())
            };

            args.iter().for_each(|a| n_args.push(Left(a)));

            let (code, _) = call_function(fid, ifid, et.location(), &name, &n_args, return_into.clone(), global_table, local_variables, global_tracker)?;
            (code, return_into)
        }
        EvaluableTokens::None => {
            (String::new(), None)
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

    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return WErr::ne(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone()),
                NameResult::Type(_) => return WErr::ne(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone()),
                NameResult::Variable(address) => {
                    if address.type_ref() != target.type_ref() {
                        return WErr::ne(
                            ExpectedDifferentType(global_table.get_type_name(address.type_ref()), global_table.get_type_name(target.type_ref())),
                            name.location().clone()
                        );
                    }
                    copy(*address.local_address(), *target.local_address(), global_table.get_size(target.type_ref()))
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            if target.type_ref().indirection().has_indirection() {
                return WErr::ne(EvalErrs::BadIndirection(target.type_ref().indirection().0, 0), literal.location().clone());
            }
            let t = global_table.get_type(*target.type_ref().type_id());

            t.instantiate_from_literal(target.local_address(), literal)?
        }
        EvaluableTokens::InfixOperator(lhs, op, rhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Infix)?;
            let signature = global_table.get_function_signature(op_fn);

            if signature.get().args().len() != 2 {
                return WErr::ne(
                    EvalErrs::InfixOpWrongArgumentCount(
                        op.operator().to_str().to_string(),
                        global_table.get_type(*lhs_type.type_id()).name().to_string(),
                        op.operator().get_method_name(PrefixOrInfixEx::Infix).unwrap(),
                        signature.get().args().len()
                    ),
                    op.location().clone()
                );
            }

            match signature.get().return_type() {
                None => {
                    return WErr::ne(OpNoReturn(global_table.get_type_name(target.type_ref())), op.location().clone())
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return WErr::ne(OpWrongReturnType(global_table.get_type_name(target.type_ref()), global_table.get_type_name(rt)), op.location().clone())
                    }
                }
            }

            let (c, _) = call_function(fid, op_fn, et.location(), &op.operator().get_method_name(PrefixOrInfixEx::Infix).unwrap(), &[Left(lhs), Left(rhs)], Some(target), global_table, local_variables, global_tracker)?;

            c
        },
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;

            if op.operator() == &OperatorTokens::Reference {
                let (mut c, val) = compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
                let Some(val) = val else {
                    return WErr::ne(ExpectedNotNone, lhs.location().clone())
                };

                if *val.type_ref() != lhs_type {
                    panic!()
                }

                c += &set_reference(op.location(), val, target, global_table)?;

                return Ok(c);
            }

            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Prefix)?;
            let signature = global_table.get_function_signature(op_fn);

            if signature.get().args().len() != 1 {
                return WErr::ne(
                    EvalErrs::InfixOpWrongArgumentCount(
                        op.operator().to_str().to_string(),
                        global_table.get_type(*lhs_type.type_id()).name().to_string(),
                        op.operator().get_method_name(PrefixOrInfixEx::Prefix).unwrap(),
                        signature.get().args().len()
                    ),
                    op.location().clone()
                );
            }

            match signature.get().return_type() {
                None => {
                    return WErr::ne(OpNoReturn(global_table.get_type_name(target.type_ref())), op.location().clone())
                }
                Some(rt) => {
                    if rt != target.type_ref() {
                        return WErr::ne(OpWrongReturnType(global_table.get_type_name(target.type_ref()), global_table.get_type_name(rt)), op.location().clone())
                    }
                }
            }

            let (c, _) = call_function(fid, op_fn, et.location(), &op.operator().get_method_name(PrefixOrInfixEx::Prefix).unwrap(), &[Left(lhs)], Some(target), global_table, local_variables, global_tracker)?;
            c
        },
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid, name) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let mut n_args = if let Some(slf) = slf.as_ref() {
                let mut v = Vec::with_capacity(args.len() + 1);
                v.push(Right(slf));
                v
            }
            else {
                Vec::with_capacity(args.len())
            };

            args.iter().for_each(|a| n_args.push(Left(a)));

            let (code, _) = call_function(fid, ifid, et.location(), &name, &n_args, Some(target), global_table, local_variables, global_tracker)?;
            code
        }
        EvaluableTokens::None => {
            return WErr::ne(EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())), et.location().clone());
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
                NameResult::Function(_) => return WErr::ne(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone()),
                NameResult::Type(_) => return WErr::ne(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone()),
                NameResult::Variable(address) => {
                    (String::new(), Some(address))
                }
            }
        },
        EvaluableTokens::Literal(_) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::InfixOperator(_, _, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::PrefixOperator(_, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(_, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::None => {
            (String::new(), None)
        }
    })
}

/// Will always return a function pointer (and self)
pub fn compile_evaluable_function_only(
    fid: FunctionID,
    name: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker
) -> Result<(Option<AddressedTypeRef>, FunctionID, String), WErr> {
    Ok(match name.token() {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(fid) => {
                    (None, fid, name.name().clone())
                }
                _ => return WErr::ne(ExpectedFunctionName, name.location().clone()),
            }
        }
        _ => return WErr::ne(ExpectedFunctionName, name.location().clone())
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

    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => return WErr::ne(EvalErrs::FunctionMustBeCalled(name.name().clone()), name.location().clone()),
                NameResult::Type(_) => return WErr::ne(EvalErrs::CannotEvalStandaloneType(name.name().clone()), name.location().clone()),
                NameResult::Variable(address) => {
                    address.type_ref().clone()
                }
            }
        },
        EvaluableTokens::Literal(literal) => {
            let tid = literal.literal().default_type();
            TypeRef::new(tid, Indirection(0))
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
            signature.get().return_type().as_ref().unwrap().clone()
        },
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;

            if op.operator() == &OperatorTokens::Reference { return Ok(lhs_type.plus_one_indirect()) }

            // code += "\n";
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Prefix)?;
            let signature = global_table.get_function_signature(op_fn);
            signature.get().return_type().as_ref().unwrap().clone()
        },
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrors::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid, _) = compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let signature = global_table.get_function_signature(ifid);
            let return_type = signature.get().return_type().clone().unwrap(); // TODO: Check type
            return_type
        }
        EvaluableTokens::None => {
            return WErr::ne(EvalErrs::ExpectedNotNone, et.location().clone());
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