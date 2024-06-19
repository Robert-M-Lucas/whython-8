pub mod types;
pub mod functions;

use crate::root::builtin::functions::register_functions;
use crate::root::builtin::types::bool::register_bool;
use crate::root::builtin::types::int::register_int;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::common::{ByteSize, FunctionID, LocalAddress, TypeID};

pub fn register_builtin(global_table: &mut GlobalDefinitionTable) {
    register_functions(global_table);
    register_int(global_table);
    register_bool(global_table);
}

pub type InlineFunctionGenerator = fn(&[LocalAddress], Option<LocalAddress>, &mut GlobalTracker, ByteSize) -> String;

const fn f_id(id: u16) -> FunctionID {
    FunctionID(-(id as isize) - 1)
}

const fn t_id(id: u16) -> TypeID {
    TypeID(-(id as isize) - 1)
}

pub trait BuiltinInlineFunction {
    fn id(&self) -> FunctionID;
    fn name(&self) -> &'static str;
    fn signature(&self) -> FunctionSignature;
    fn inline(&self) -> InlineFunctionGenerator;
    fn requirements(&self) -> Option<Vec<String>> {
        None
    }
    fn parent_type(&self) -> Option<TypeID>;
}