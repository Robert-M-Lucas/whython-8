use crate::root::assembler::assembly_builder::AssemblyBuilder;
use crate::root::shared::common::{ByteSize, LocalAddress};


// pub fn get_jump_tag(id: FunctionID, jump_id: usize) -> String {
//     if id.is_main() {
//         return format!("main.{jump_id}");
//     }
//
//     let id = id.0;
//     if id > 0 {
//         format!(".{id}.{jump_id}")
//     }
//     else {
//         format!("._{}.{jump_id}", -id)
//     }
// }

/// Align a number of bytes to the next multiple of 16
pub fn align_16_bytes(bytes: ByteSize) -> ByteSize {
    let bytes = bytes.0;
    if bytes % 16 == 0 {
        ByteSize(bytes)
    } else {
        ByteSize(bytes + (16 - (bytes % 16)))
    }
}

/// Align a number of bytes to the next multiple of 16 + 8
pub fn align_16_bytes_plus_8(bytes: ByteSize) -> ByteSize {
    let bytes = bytes.0;
    if bytes % 16 == 8 {
        ByteSize(bytes)
    } else {
        ByteSize(bytes + (16 % (24 - (bytes % 16))))
    }
}

// pub fn get_qword_stack_pointer(address: &LocalAddress) -> String {
//     let address = address.0;
//
//     if address >= 0 {
//         format!("qword [rbp+{address}]")
//     } else {
//         format!("qword [rbp{address}]")
//     }
// }
//
// pub fn get_dword_stack_pointer(address: &LocalAddress) -> String {
//     let address = address.0;
//
//     if address >= 0 {
//         format!("dword [rbp+{address}]")
//     } else {
//         format!("dword [rbp{address}]")
//     }
// }
//
// pub fn get_word_stack_pointer(address: &LocalAddress) -> String {
//     let address = address.0;
//
//     if address >= 0 {
//         format!("word [rbp+{address}]")
//     } else {
//         format!("word [rbp{address}]")
//     }
// }
//
// pub fn get_byte_stack_pointer(address: &LocalAddress) -> String {
//     let address = address.0;
//
//     if address >= 0 {
//         format!("byte [rbp+{address}]")
//     } else {
//         format!("byte [rbp{address}]")
//     }
// }

/// Copies data. Expects `from` to be the address of a pointer pointing to the data to move
/// and `to` to be the target
pub fn copy_from_indirect(from: LocalAddress, to: LocalAddress, amount: ByteSize) -> String {
    if amount == ByteSize(0) { return String::new(); }

    let to = to.0;
    let mut written = 0;
    let mut output = AssemblyBuilder::new();

    output.line(&format!("mov rdx, qword {from}"));

    loop {
        let to_write = amount.0 - written;
        if to_write >= 8 {
            output.line(&format!("mov rax, qword [rdx+{written}]", ));
            output.line(&format!("mov qword {}, rax", &LocalAddress(to + written as isize)));
            written += 8;
        }
        else if to_write >= 4 {
            output.line(&format!("mov eax, dword [rdx+{written}]", ));
            output.line(&format!("mov dword {}, eax", &LocalAddress(to + written as isize)));
            written += 4;
        }
        else if to_write >= 2 {
            output.line(&format!("mov ax, word [rdx+{written}]", ));
            output.line(&format!("mov word {}, ax", &LocalAddress(to + written as isize)));
            written += 2;
        }
        else if to_write >= 1 {
            output.line(&format!("mov al, byte [rdx+{written}]", ));
            output.line(&format!("mov byte {}, al", &LocalAddress(to + written as isize)));
            written += 1;
        }
        else {
            break;
        }
        if written == amount.0 { break; }
    }
    output.finish()
}

/// Copies data. Expects `from` to be the address of the data to move
/// and `to` to be a pointer to the target
pub fn copy_to_indirect(from: LocalAddress, to: LocalAddress, amount: ByteSize) -> String {
    if amount == ByteSize(0) { return String::new(); }

    let from = from.0;
    let mut written = 0;

    let mut output = AssemblyBuilder::new();
    output.line(&format!("mov rdx, qword {to}"));

    loop {
        let to_write = amount.0 - written;
        if to_write >= 8 {
            output.line(&format!("mov rax, qword {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov qword [rdx+{written}], rax"));
            written += 8;
        }
        else if to_write >= 4 {
            output.line(&format!("mov eax, dword {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov dword [rdx+{written}], eax"));
            written += 4;
        }
        else if to_write >= 2 {
            output.line(&format!("mov ax, word {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov word [rdx+{written}], ax"));
            written += 2;
        }
        else if to_write >= 1 {
            output.line(&format!("mov al, byte {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov byte [rdx+{written}], byte"));
            written += 1;
        }
        else {
            break;
        }
        if written == amount.0 { break; }
    }
    output.finish()
}

/// Copies data. Expects `from` to be the address of the data to move
/// and `to` to be the target
pub fn copy(from: LocalAddress, to: LocalAddress, amount: ByteSize) -> String {
    if amount == ByteSize(0) { return String::new(); }

    let from = from.0;
    let to = to.0;
    let mut written = 0;

    let mut output = AssemblyBuilder::new();

    loop {
        let to_write = amount.0 - written;
        if to_write >= 8 {
            output.line(&format!("mov rax, qword {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov qword {}, rax", &LocalAddress(to + written as isize)));
            written += 8;
        }
        else if to_write >= 4 {
            output.line(&format!("mov eax, dword {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov dword {}, eax", &LocalAddress(to + written as isize)));
            written += 4;
        }
        else if to_write >= 2 {
            output.line(&format!("mov ax, word {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov word {}, ax", &LocalAddress(to + written as isize)));
            written += 2;
        }
        else if to_write >= 1 {
            output.line(&format!("mov al, byte {}", LocalAddress(from + written as isize)));
            output.line(&format!("mov byte {}, al", &LocalAddress(to + written as isize)));
            written += 1;
        }
        else {
            break;
        }
        if written == amount.0 { break; }
    }
    output.finish()
}