use crate::root::compiler::evaluation::function_only;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::evaluable_errors::EvalErrs::ExpectedReference;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::parser::parse_function::parse_operator::{OperatorTokens, PrefixOrInfixEx};
use crate::root::shared::common::{FunctionID, Indirection, TypeRef};

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
            let (slf, ifid, _) = function_only::compile_evaluable_function_only(fid, inner, local_variables, global_table, global_tracker)?;

            let signature = global_table.get_function_signature(ifid);
            let return_type = signature.get().return_type().clone().unwrap(); // TODO: Check type
            return_type
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            let t = global_table.resolve_to_type_ref(struct_init.name())?;
            debug_assert!(!t.indirection().has_indirection());
            t
        }
        EvaluableTokens::None => {
            return WErr::ne(EvalErrs::ExpectedNotNone, et.location().clone());
        }
    })
}