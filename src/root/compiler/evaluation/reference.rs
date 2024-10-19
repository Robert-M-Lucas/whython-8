use crate::root::assembler::assembly_builder::Assembly;
use crate::root::compiler::evaluation::new::compile_evaluable_new;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{AddressedTypeRef, FunctionID};

/// Evaluates `et` attempting to return a reference to an existing variable as opposed to allocating
pub fn compile_evaluable_reference(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(Assembly, Option<AddressedTypeRef>), WErr> {
    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(
                name,
                None,
                containing_class.as_ref(),
                local_variables,
                global_tracker,
            )? {
                NameResult::Function(_) => {
                    // Name was function (no call at the end)
                    return WErr::ne(
                        EvalErrs::FunctionMustBeCalled(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Type(_) => {
                    // Name was a type (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvalStandaloneType(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::File(_) => {
                    // Name was a file (not a value)
                    return WErr::ne(
                        EvalErrs::CannotEvaluateStandaloneImportedFile(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Variable(address) => (String::new(), Some(address)),
            }
        }
        EvaluableTokens::Literal(_) => {
            // Cannot get an address without instantiation
            compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::InfixOperator(_, _, _) => {
            // Cannot get an address without instantiation
            compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::PrefixOperator(_, _) => {
            // Cannot get an address without instantiation
            compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        // Cannot get an address without instantiation
        EvaluableTokens::DynamicAccess {
            parent: _,
            section: _,
        } => compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::StaticAccess {
            parent: _,
            section: n,
        } => {
            // Constant attributes do not exist - if it's a method, it must be called
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(n.name().clone()),
                n.location().clone(),
            )
        }
        // Cannot get an address without instantiation
        EvaluableTokens::FunctionCall {
            function: _,
            args: _,
        } => compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?,
        EvaluableTokens::StructInitialiser(_struct_init) => {
            // Cannot get an address without instantiation
            compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::None => (String::new(), None),
    })
}
