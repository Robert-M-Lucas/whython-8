use crate::root::errors::WErr;
use crate::root::parser::parse_function::parse_literal::LiteralToken;
use crate::root::parser::parse_name::SimpleNameToken;
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID, TypeRef};

/// A type
pub trait Type {
    fn id(&self) -> TypeID;

    fn size(&self) -> ByteSize;

    fn name(&self) -> &str;

    fn get_attributes(&self) -> Result<&[(ByteSize, SimpleNameToken, TypeRef)], WErr> {
        Err(todo!())
    }

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> Result<String, WErr>;
}
