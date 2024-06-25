use crate::root::compiler::assembly::utils::copy_from_indirect;
use crate::root::errors::evaluable_errors::EvalErrs::{ExpectedDifferentType, OpWrongReturnType};
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::parser::parse::Location;
use crate::root::shared::common::AddressedTypeRef;

/// Sets `into` to the address of `to_ref`
pub fn set_reference(location: &Location, to_ref: AddressedTypeRef, into: AddressedTypeRef, global_table: &GlobalDefinitionTable) -> Result<String, WErr> {
    let new_type = to_ref.type_ref().type_id().with_indirection(to_ref.type_ref().indirection().0 + 1);
    if new_type != *into.type_ref() {
        return WErr::ne(OpWrongReturnType(global_table.get_type_name(into.type_ref()), global_table.get_type_name(&new_type)), location.clone());
    }

    Ok(format!("    mov rax, rbp
    add rax, {}
    mov qword {}, rax\n", to_ref.local_address().0, into.local_address()))
}

/// Sets `into` to the value pointed to by `to_deref`
pub fn set_deref(location: &Location, to_deref: AddressedTypeRef, into: AddressedTypeRef, global_table: &mut GlobalDefinitionTable) -> Result<String, WErr> {
    let expected = into.type_ref().plus_one_indirect();
    if to_deref.type_ref() != &expected {
        return WErr::ne(ExpectedDifferentType(global_table.get_type_name(&expected), global_table.get_type_name(to_deref.type_ref())), location.clone());
    }
    Ok(copy_from_indirect(*to_deref.local_address(), *into.local_address(), global_table.get_size(into.type_ref())))
}