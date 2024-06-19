use crate::root::errors::WErr;
use crate::root::parser::parse_function::parse_literal::LiteralToken;
use crate::root::shared::common::{ByteSize, LocalAddress, TypeID};

pub trait Type {
    fn id(&self) -> TypeID;

    fn size(&self) -> ByteSize;

    fn name(&self) -> &str;

    fn instantiate_from_literal(&self, location: &LocalAddress, literal: &LiteralToken) -> Result<String, WErr>;
}
