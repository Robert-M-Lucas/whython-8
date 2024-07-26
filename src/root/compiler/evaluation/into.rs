use std::any::Any;
use either::{Left, Right};
use itertools::Itertools;
use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::builtin::core::referencing::{set_deref, set_reference};
use crate::root::compiler::assembly::utils::copy;
use crate::root::compiler::compile_function_call::call_function;
use crate::root::compiler::evaluation::{function_only, reference, type_only};
use crate::root::compiler::evaluation::reference::compile_evaluable_reference;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedDifferentType, ExpectedNotNone, ExpectedType, OpNoReturn, OpWrongReturnType};
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::shared::common::{AddressedTypeRef, FunctionID, LocalAddress};
use crate::root::shared::types::Type;

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

            let lhs_type = type_only::compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;

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
            let lhs_type = type_only::compile_evaluable_type_only(fid, lhs, local_variables, global_table, global_tracker)?;

            match op.operator() {
                OperatorTokens::Reference => {
                    let (mut c, val) = reference::compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
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
                    let (mut c, val) = reference::compile_evaluable_reference(fid, lhs, local_variables, global_table, global_tracker)?;
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
        EvaluableTokens::DynamicAccess(inner, access) => {
            let mut ab = AssemblyBuilder::new();
            let (c, inner) = compile_evaluable_reference(fid, inner, local_variables, global_table, global_tracker)?;
            ab.other(&c);

            let Some(inner) = inner else { todo!() };

            let t = global_table.get_type(*inner.type_ref().type_id());
            let attribs = t.get_attributes()?;

            let mut found_offset = None;

            for (offset, name, t) in attribs {
                if name.name() == access.name() {
                    if t != target.type_ref() {
                        todo!()
                    }
                    found_offset = Some(*offset);
                }
            }

            let Some(found_offset) = found_offset else { todo!() };

            ab.other(&copy(LocalAddress(inner.local_address().0 + found_offset.0 as isize), *target.local_address(), global_table.get_size(target.type_ref())));

            ab.finish()
        },
        EvaluableTokens::StaticAccess(_, n) => return WErr::ne(NRErrs::CannotFindConstantAttribute(n.name().clone()), n.location().clone()), // Accessed methods must be called
        EvaluableTokens::FunctionCall(inner, args) => {
            let (slf, ifid, name) = function_only::compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

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
        EvaluableTokens::StructInitialiser(struct_init) => {
            let t = global_table.resolve_to_type_ref(struct_init.name())?;
            debug_assert!(!t.indirection().has_indirection());
            if &t != target.type_ref() {
                todo!();
            }
            let tt = global_table.get_type(t.type_id().clone());
            let attributes = tt.get_attributes()?.iter().map(|x| x.clone()).collect_vec();
            let give_attrs = struct_init.contents();

            if attributes.len() != give_attrs.len() {
                todo!()
            }

            let mut code = AssemblyBuilder::new();

            // TODO: Doable without clone?
            for ((offset, t_name, t_type), (name, val)) in attributes.iter().zip(give_attrs.iter()) {
                if t_name.name() != name.name() {
                    todo!()
                }

                let new_addr = AddressedTypeRef::new(LocalAddress(target.local_address().0 + offset.0 as isize), t_type.clone());
                code.other(&compile_evaluable_into(fid, val, new_addr, local_variables, global_table, global_tracker)?);
            }

            code.finish()
        }
        EvaluableTokens::None => {
            return WErr::ne(EvalErrs::ExpectedType(global_table.get_type_name(target.type_ref())), et.location().clone());
        }
    })
}