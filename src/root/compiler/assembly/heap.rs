use crate::root::builtin::{BuiltinInlineFunction, InlineFunctionGenerator};
use crate::root::compiler::local_variable_table::LocalVariableTable;
use crate::root::name_resolver::name_resolvers::GlobalDefinitionTable;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::parser::parse_parameters::SelfType;
use crate::root::shared::common::{AddressedTypeRef, FunctionID, TypeID, TypeRef};

pub fn heap_alloc(
    t: TypeRef,
    global_table: &mut GlobalDefinitionTable,
    local_variable_table: &mut LocalVariableTable,
) -> (String, AddressedTypeRef) {
    let size = global_table.get_size(&t).0;
    let sz = local_variable_table.stack_size().0;
    let output =
        global_table.add_local_variable_unnamed_base(t.plus_one_indirect(), local_variable_table);

    (
        format!(
            "    mov rdi, {size}
    sub rsp, {sz}
    extern malloc
    call malloc
    add rsp, {sz}
    mov qword {}, rax\n",
            output.local_address()
        ),
        output,
    )
}

pub struct FreeFunction {
    id: FunctionID,
    parent_type: TypeID,
}

impl BuiltinInlineFunction for FreeFunction {
    fn id(&self) -> FunctionID {
        self.id
    }

    fn name(&self) -> &'static str {
        "free"
    }

    fn signature(&self) -> FunctionSignature {
        FunctionSignature::new(
            SelfType::None,
            vec![(
                SimpleNameToken::new_builtin("heap_pointer".to_string()),
                self.parent_type.with_indirection_single(1),
            )],
            None,
        )
    }

    fn inline(&self) -> InlineFunctionGenerator {
        |args, _, _, sz| -> String {
            let to_free = &args[0];
            format!(
                "    mov rdi, qword {to_free}
    sub rsp, {sz}
    extern free
    call free
    add rsp, {sz}\n"
            )
        }
    }

    fn parent_type(&self) -> Option<TypeID> {
        Some(self.parent_type)
    }
}

pub fn free_function(t: TypeID, f: FunctionID) -> FreeFunction {
    FreeFunction {
        id: f,
        parent_type: t,
    }
}
