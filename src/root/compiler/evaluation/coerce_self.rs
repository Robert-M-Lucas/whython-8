use crate::root::builtin::core::referencing::set_reference;
use crate::root::compiler::assembly::utils::copy_from_indirect;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::location::Location;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{AddressedTypeRef, Indirection};

pub fn coerce_self(
    current_self: AddressedTypeRef,
    self_type: SelfType,
    global_table: &mut GlobalDefinitionTable,
    local_variables: &mut LocalVariableTable,
) -> Result<(String, AddressedTypeRef), WErr> {
    Ok(match self_type {
        SelfType::None => (String::new(), current_self),
        SelfType::CopySelf => {
            if current_self.type_ref().indirection().has_indirection() {
                let new_self = global_table.add_local_variable_unnamed_base(
                    current_self.type_ref().type_id().immediate(),
                    local_variables,
                );

                (
                    copy_from_indirect(
                        *current_self.local_address(),
                        *new_self.local_address(),
                        global_table.get_size(new_self.type_ref()),
                        Indirection(current_self.type_ref().indirection().0),
                    ),
                    new_self,
                )
            } else {
                (String::new(), current_self)
            }
        }
        SelfType::RefSelf => {
            if !current_self.type_ref().indirection().has_indirection() {
                let new_self = global_table.add_local_variable_unnamed_base(
                    current_self.type_ref().plus_one_indirect(),
                    local_variables,
                );
                (
                    set_reference(
                        &Location::builtin(),
                        current_self,
                        new_self.clone(),
                        global_table,
                    )?,
                    new_self,
                )
            } else if *current_self.type_ref().indirection() == Indirection(1) {
                (String::new(), current_self)
            } else {
                let new_self = global_table.add_local_variable_unnamed_base(
                    current_self.type_ref().type_id().with_indirection(1),
                    local_variables,
                );

                (
                    copy_from_indirect(
                        *current_self.local_address(),
                        *new_self.local_address(),
                        global_table.get_size(new_self.type_ref()),
                        Indirection(current_self.type_ref().indirection().0 - 1), // Want a ref, not inner type
                    ),
                    new_self,
                )
            }
        }
    })
}
