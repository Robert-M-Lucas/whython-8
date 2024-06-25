use std::any::Any;
use either::{Left, Right};
use itertools::Itertools;
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::utils::{copy, copy_from_indirect, copy_to_indirect};
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedDifferentType, ExpectedFunctionName, ExpectedNotNone, ExpectedReference, ExpectedType, OpNoReturn, OpWrongReturnType};
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};
use crate::root::shared::common::AddressedTypeRef;

/// Error on an empty address
fn expect_addr(r: (String, Option<AddressedTypeRef>)) -> Result<(String, AddressedTypeRef), WErr> {
    Ok((r.0, r.1.unwrap())) // TODO
}

/// Evaluates `et` into a new address
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
            match op.operator() {
                OperatorTokens::Assign => {
                    let (mut c, into) = compile_evaluable(fid, lhs, local_variables, global_table, global_tracker)?;
                    let Some(into) = into else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone())
                    };
                    if !into.type_ref().indirection().has_indirection() {
                        return WErr::ne(ExpectedReference(global_table.get_type_name(into.type_ref())), lhs.location().clone());
                    }

                    let val = global_table.add_local_variable_unnamed_base(into.type_ref().minus_one_indirect(), local_variables);
                    c += &compile_evaluable_into(fid, rhs, val.clone(), local_variables, global_table, global_tracker)?;

                    c += &copy_to_indirect(*val.local_address(), *into.local_address(), global_table.get_size(val.type_ref()));
                    return Ok((c, None));
                }
                _ => {}
            };

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
        EvaluableTokens::PrefixOperator(op, lhs) => {
            let lhs_type = compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;

            match op.operator() {
                OperatorTokens::Reference => {
                    let (mut c, val) = compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
                    let Some(val) = val else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone())
                    };

                    if *val.type_ref() != lhs_type {
                        panic!()
                    }
                    let target = global_table.add_local_variable_unnamed_base(val.type_ref().plus_one_indirect(), local_variables);
                    c += &set_reference(op.location(), val, target.clone(), global_table)?;
                    return Ok((c, Some(target)));
                }
                OperatorTokens::Multiply => {
                    let (mut c, val) = compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
                    let Some(val) = val else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone());
                    };
                    if !val.type_ref().indirection().has_indirection() {
                        return WErr::ne(ExpectedReference(global_table.get_type_name(val.type_ref())), lhs.location().clone());
                    }
                    let target = global_table.add_local_variable_unnamed_base(val.type_ref().minus_one_indirect(), local_variables);
                    c += &set_deref(lhs.location(), val, target.clone(), global_table)?;
                    return Ok((c, Some(target)));
                }
                _ => {}
            };

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

            let return_type = signature.get().return_type().clone();
            let return_into = return_type.map(|rt| global_table.add_local_variable_unnamed_base(rt.clone(), local_variables));

            let (c, _) = call_function(fid, op_fn, et.location(), &op.operator().get_method_name(PrefixOrInfixEx::Prefix).unwrap(), &[Left(lhs)], return_into.clone(), global_table, local_variables, global_tracker)?;
            (c, return_into)
        },
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrs::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
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


/// Evaluates `et` putting the result into `target`
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
                            ExpectedDifferentType(global_table.get_type_name(target.type_ref()), global_table.get_type_name(address.type_ref())),
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
            match op.operator() {
                OperatorTokens::Assign => {
                    return WErr::ne(ExpectedType(global_table.get_type_name(target.type_ref())), et.location().clone());
                }
                _ => {}
            };

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

            match op.operator() {
                OperatorTokens::Reference => {
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
                OperatorTokens::Multiply => {
                    let (mut c, val) = compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
                    let Some(val) = val else {
                        return WErr::ne(ExpectedNotNone, lhs.location().clone());
                    };

                    c += &set_deref(lhs.location(), val, target, global_table)?;
                    return Ok(c);
                }
                _ => {}
            };

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
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrs::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
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

/// Evaluates `et` attempting to return a reference to an existing variable as opposed to allocating
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
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrs::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(_, _) => compile_evaluable(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::None => {
            (String::new(), None)
        }
    })
}

/// Evaluates `name` into a `FunctionID`
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

/// Evaluates the type `et` evaluates to. Does not generate any assembly.
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

            match op.operator() {
                OperatorTokens::Reference => return Ok(lhs_type.plus_one_indirect()),
                OperatorTokens::Multiply => {
                    if !lhs_type.indirection().has_indirection() {
                        return WErr::ne(ExpectedReference(global_table.get_type_name(&lhs_type)), lhs.location().clone());
                    }
                    return Ok(lhs_type.minus_one_indirect())
                }
                _ => {}
            };

            // code += "\n";
            let op_fn = global_table.get_operator_function(*lhs_type.type_id(), op, PrefixOrInfixEx::Prefix)?;
            let signature = global_table.get_function_signature(op_fn);
            signature.get().return_type().as_ref().unwrap().clone()
        },
        EvaluableTokens::DynamicAccess(_, _) => todo!(), // Accessed methods must be called
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrs::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
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

// Will ignore everything other than type with a target type
// pub fn compile_evaluable_type_only_into(
//     fid: FunctionID,
//     et: &EvaluableToken,
//     target: TypeRef,
//     local_variables: &mut LocalVariableTable,
//     global_table: &mut GlobalDefinitionTable,
//     global_tracker: &mut GlobalTracker
// ) -> Result<bool, WErr> {
//
//     let et = et.token();
//
//     todo!()
// }