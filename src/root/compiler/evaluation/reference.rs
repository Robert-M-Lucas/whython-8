use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::compiler::evaluation::new;
use crate::root::compiler::evaluation::new::compile_evaluable_new;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::name_resolver_errors::NRErrs;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable, NameResult};
use crate::root::parser::parse_function::parse_evaluable::{EvaluableToken, EvaluableTokens};
use crate::root::shared::common::{AddressedTypeRef, FunctionID, LocalAddress};

/// Evaluates `et` attempting to return a reference to an existing variable as opposed to allocating
pub fn compile_evaluable_reference(
    fid: FunctionID,
    et: &EvaluableToken,
    local_variables: &mut LocalVariableTable,
    global_table: &mut GlobalDefinitionTable,
    global_tracker: &mut GlobalTracker,
) -> Result<(String, Option<AddressedTypeRef>), WErr> {
    let ets = et.token();

    Ok(match ets {
        EvaluableTokens::Name(name, containing_class) => {
            match global_table.resolve_name(name, containing_class.as_ref(), local_variables)? {
                NameResult::Function(_) => {
                    return WErr::ne(
                        EvalErrs::FunctionMustBeCalled(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Type(_) => {
                    return WErr::ne(
                        EvalErrs::CannotEvalStandaloneType(name.name().clone()),
                        name.location().clone(),
                    )
                }
                NameResult::Variable(address) => (String::new(), Some(address)),
            }
        }
        EvaluableTokens::Literal(_) => {
            new::compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::InfixOperator(_, _, _) => {
            compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::PrefixOperator(_, _) => {
            new::compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::DynamicAccess(inner, access) => {
            let mut ab = AssemblyBuilder::new();
            let (c, inner) = compile_evaluable_reference(
                fid,
                inner,
                local_variables,
                global_table,
                global_tracker,
            )?;
            ab.other(&c);

            let Some(inner) = inner else { todo!() };

            let t = global_table.get_type(*inner.type_ref().type_id());
            let attribs = t.get_attributes()?;

            let mut out = None;

            for (offset, name, t) in attribs {
                if name.name() == access.name() {
                    out = Some(AddressedTypeRef::new(
                        LocalAddress(inner.local_address().0 + offset.0 as isize),
                        t.clone(),
                    ));
                    break;
                }
            }

            if let Some(out) = out {
                (ab.finish(), Some(out))
            } else {
                todo!()
            }
        }
        EvaluableTokens::StaticAccess(_, n) => {
            return WErr::ne(
                NRErrs::CannotFindConstantAttribute(n.name().clone()),
                n.location().clone(),
            )
        } // Accessed methods must be called
        EvaluableTokens::FunctionCall(_, _) => {
            new::compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::StructInitialiser(struct_init) => {
            new::compile_evaluable_new(fid, et, local_variables, global_table, global_tracker)?
        }
        EvaluableTokens::None => (String::new(), None),
    })
}
