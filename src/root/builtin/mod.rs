pub mod core;
pub mod functions;
pub mod types;

use crate::root::builtin::functions::register_functions;
use crate::root::builtin::types::bool::register_bool;
use crate::root::builtin::types::int::register_int;
use crate::root::compiler::global_tracker::GlobalTracker;
use crate::root::name_resolver::name_resolvers::GlobalTable;
use crate::root::name_resolver::resolve_function_signatures::FunctionSignature;
use crate::root::shared::common::{ByteSize, FunctionID, LocalAddress, TypeID};

/// Registers all the builtin types and their relevant functions
pub fn register_builtin(global_table: &mut GlobalTable) {
    register_functions(global_table);
    register_int(global_table);
    register_bool(global_table);
}

/// Function that takes context and generates inline assembly to be used within a functions
pub type InlineFnGenerator =
    fn(&[LocalAddress], Option<LocalAddress>, &mut GlobalTracker, ByteSize) -> String;

/// Converts a u16 unique ID to a non-zero, negative `FunctionID`
pub const fn f_id(id: u16) -> FunctionID {
    FunctionID(-(id as isize) - 1)
}

/// Converts a u16 unique ID to a non-zero, negative `FunctionID`
pub const fn t_id(id: u16) -> TypeID {
    TypeID(-(id as isize) - 1)
}

/// Trait for a 'function' implemented as inline assembly
pub trait BuiltinInlineFunction {
    fn id(&self) -> FunctionID;
    fn name(&self) -> &'static str;
    fn signature(&self) -> FunctionSignature;
    fn inline(&self) -> InlineFnGenerator;
    fn parent_type(&self) -> Option<TypeID>;
}
