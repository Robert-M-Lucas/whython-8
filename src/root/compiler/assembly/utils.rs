use crate::root::shared::types::FunctionID;

pub fn get_function_tag(id: FunctionID) -> String {
    let id = id.0;
    if id == 0 {
        "main".to_string()
    }
    else if id > 0 {
        format!("_{id}")
    }
    else {
        format!("__{}", -id)
    }
}

pub fn get_jump_tag(id: FunctionID, jump_id: usize) -> String {
    let id = id.0;
    if id == 0 {
        format!("main.{jump_id}")
    }
    else if id > 0 {
        format!(".{id}.{jump_id}")
    }
    else {
        format!("._{}.{jump_id}", -id)
    }
}