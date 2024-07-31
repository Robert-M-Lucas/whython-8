use crate::root::compiler::compiler_errors::CompErrs;
use crate::root::errors::evaluable_errors::EvalErrs;
use crate::root::errors::WErr;
use crate::root::parser::parse::Location;
use crate::root::parser::parse_function::parse_literal::LiteralToken;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID, TypeRef};

/// A type
pub trait Type {
    fn id(&self) -> TypeID;

    fn size(&self) -> ByteSize;

    fn name(&self) -> &str;

    fn get_attributes(&self, location: &Location) -> Result<&[(ByteSize, SimpleNameToken, TypeRef)], WErr> {
        WErr::ne(EvalErrs::TypeDoesntHaveAttributes(self.name().to_string()), location.clone())
    }

    fn instantiate_from_literal(
        &self,
        location: &LocalAddress,
        literal: &LiteralToken,
    ) -> Result<String, WErr>;
}
