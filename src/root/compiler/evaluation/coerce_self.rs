use crate::root::builtin::core::referencing::set_reference;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{AddressedTypeRef, Indirection};

pub fn coerce_self(current_self: AddressedTypeRef, self_type: SelfType, global_table: &mut GlobalDefinitionTable, local_variables: &mut LocalVariableTable) -> Result<(String, AddressedTypeRef), WErr> {
    Ok(match self_type {
        SelfType::None => {
            (String::new(), current_self)
        }
        SelfType::CopySelf => {
            if current_self.type_ref().indirection().has_indirection() {
                todo!()
            }
            (String::new(), current_self)
        }
        SelfType::RefSelf => {
            if !current_self.type_ref().indirection().has_indirection() {
                let new_self = global_table.add_local_variable_unnamed_base(current_self.type_ref().plus_one_indirect(), local_variables);
                (set_reference(&Location::builtin(), current_self, new_self.clone(), global_table)?, new_self)
            }
            else if *current_self.type_ref().indirection() == Indirection(1) {
                (String::new(), current_self)
            }
            else {
                todo!()
            }
        }
    })
}