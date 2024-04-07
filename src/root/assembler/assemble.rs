use crate::root::compiler::compile_functions::Function;
use std::fs;

pub fn generate_assembly(output: &str, functions: Vec<Box<dyn Function>>) {
    let mut out = String::from(
        "    global main
    extern ExitProcess
    extern GetStdHandle
    extern WriteFile
    extern HeapAlloc
    extern HeapFree
    extern GetProcessHeap
    extern printf
    section .text\n",
    );
    for f in functions {
        out += "\r\n";
        out += &(f.get_asm());
    }
    
    out += "\nformatStr:
	db `The int is %d\\n`,0";

    fs::write(format!("{output}.asm"), out).expect("Failed to write assembly to file");
}
