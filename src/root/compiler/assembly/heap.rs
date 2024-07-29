use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::shared::common::{AddressedTypeRef, TypeRef};

pub fn heap_alloc(t: TypeRef, global_table: &mut GlobalDefinitionTable, local_variable_table: &mut LocalVariableTable) -> (String, AddressedTypeRef) {
    let size = global_table.get_size(&t).0;
    let sz = local_variable_table.stack_size().0;
    let output = global_table.add_local_variable_unnamed_base(t.plus_one_indirect(), local_variable_table);

    (format!("    mov rdi, {size}
    sub rsp, {sz}
    extern malloc
    call malloc
    sub rsp, {sz}
    mov qword {}, rax\n", output.local_address()), output)

}
