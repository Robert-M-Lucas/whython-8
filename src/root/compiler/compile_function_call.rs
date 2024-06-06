use crate::root::errors::WErr;
use crate::root::shared::common::{FunctionID, LocalAddress};

pub fn call_function(fid: FunctionID, arguments: &[LocalAddress], return_address: Option<LocalAddress>) -> Result<String, WErr> {
    todo!()
}