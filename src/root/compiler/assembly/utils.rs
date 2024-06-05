use std::fmt::format;
use crate::root::shared::common::{ByteSize, FunctionID, LocalAddress};

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

pub fn get_qword_stack_pointer(address: &LocalAddress) -> String {
    let address = address.0;

    if address >= 0 {
        format!("qword [rbp+{address}]")
    } else {
        format!("qword [rbp{address}]")
    }
}

pub fn get_dword_stack_pointer(address: &LocalAddress) -> String {
    let address = address.0;

    if address >= 0 {
        format!("dword [rbp+{address}]")
    } else {
        format!("dword [rbp{address}]")
    }
}

pub fn get_word_stack_pointer(address: &LocalAddress) -> String {
    let address = address.0;

    if address >= 0 {
        format!("word [rbp+{address}]")
    } else {
        format!("word [rbp{address}]")
    }
}

pub fn get_byte_stack_pointer(address: &LocalAddress) -> String {
    let address = address.0;

    if address >= 0 {
        format!("byte [rbp+{address}]")
    } else {
        format!("byte [rbp{address}]")
    }
}

pub fn copy(from: LocalAddress, to: LocalAddress, amount: ByteSize) -> String {
    if amount == ByteSize(0) { return String::new(); }

    let from = from.0;
    let to = to.0;
    let mut written = 0;

    let mut output = String::new();

    loop {
        let to_write = amount.0 - written;
        if to_write >= 8 {
            output += &format!("    mov rax, {}\n", get_qword_stack_pointer(&LocalAddress(from + written as isize)));
            output += &format!("    mov {}, rax", get_qword_stack_pointer(&LocalAddress(to + written as isize)));
            written += 8;
        }
        else if to_write >= 4 {
            output += &format!("    mov eax, {}\n", get_dword_stack_pointer(&LocalAddress(from + written as isize)));
            output += &format!("    mov {}, eax", get_dword_stack_pointer(&LocalAddress(to + written as isize)));
            written += 4;
        }
        else if to_write >= 2 {
            output += &format!("    mov ax, {}\n", get_word_stack_pointer(&LocalAddress(from + written as isize)));
            output += &format!("    mov {}, ax", get_word_stack_pointer(&LocalAddress(to + written as isize)));
            written += 2;
        }
        else if to_write >= 1 {
            output += &format!("    mov ah, {}\n", get_byte_stack_pointer(&LocalAddress(from + written as isize)));
            output += &format!("    mov {}, ah", get_byte_stack_pointer(&LocalAddress(to + written as isize)));
            written += 1;
        }
        else {
            break;
        }
        if written == amount.0 { break; }
        output += "\n";
    }

    output
}