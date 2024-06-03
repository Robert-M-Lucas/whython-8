use crate::root::shared::types::{ByteSize, FunctionID};

pub fn get_function_tag(id: FunctionID) -> String {
    if id.is_main() {
        return "main".to_string();
    }

    let id = id.0;
    if id > 0 {
        format!("_{id}")
    }
    else {
        format!("__{}", -id)
    }
}

pub fn get_jump_tag(id: FunctionID, jump_id: usize) -> String {
    if id.is_main() {
        return format!("main.{jump_id}");
    }

    let id = id.0;
    if id > 0 {
        format!(".{id}.{jump_id}")
    }
    else {
        format!("._{}.{jump_id}", -id)
    }
}

pub fn align_16_bytes(bytes: ByteSize) -> ByteSize {
    let bytes = bytes.0;
    if bytes % 16 == 0 {
        ByteSize(bytes)
    } else {
        ByteSize(bytes + (16 - (bytes % 16)))
    }
}

pub fn align_16_bytes_plus_8(bytes: ByteSize) -> ByteSize {
    let bytes = bytes.0;
    if bytes % 16 == 8 {
        ByteSize(bytes)
    } else {
        ByteSize(bytes + (16 % (24 - (bytes % 16))))
    }
}