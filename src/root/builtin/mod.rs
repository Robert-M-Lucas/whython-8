pub mod int;

use crate::root::builtin::int::{IntType, register_int};
use crate::root::errors::WErr;
use crate::root::name_resolver::name_resolvers::{GlobalDefinitionTable};
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::common::{FunctionID, LocalAddress, TypeID};
use crate::root::shared::types::Type;

pub fn register_builtin(global_table: &mut GlobalDefinitionTable) {
    register_int(global_table);
}

pub type InlineFunctionGenerator = fn(&[LocalAddress], Option<LocalAddress>) -> String;

pub trait BuiltinInlineFunction {
    fn id(&self) -> FunctionID;
    fn name(&self) -> &'static str;
    fn signature(&self) -> FunctionSignature;
    fn inline(&self) -> InlineFunctionGenerator;
    fn parent_type(&self) -> Option<TypeID>;
}