use crate::root::parser::parse::parse;
// use crate::root::assembler::assemble::generate_assembly;
// use crate::root::name_resolver::processor::process;
use crate::time;
use clap::Parser;
use color_print::cprintln;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::root::compiler::compile::compile;
use crate::root::name_resolver::resolve::resolve;
use shared::common::ByteSize;
use crate::root::errors::WError;
use crate::root::runner::{assemble, link_gcc, run};

// #[cfg(target_os = "windows")]
// use crate::root::runner::run;
// #[cfg(target_os = "windows")]
// use runner::link;
//
// #[cfg(target_os = "linux")]
// use crate::root::runner::run_wine_experimental;
// #[cfg(target_os = "linux")]
// use runner::link_gcc_experimental;
// use crate::root::parser::parse::parse;

pub mod parser;
pub mod runner;
pub mod utils;
pub mod name_resolver;
pub mod builtin;
pub mod shared;
pub mod compiler;
pub mod assembler;
pub mod errors;

pub const POINTER_SIZE: ByteSize = ByteSize(8);

/// Compiler for Whython files (.why)
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main input file
    #[arg(short, long, default_value = "main.why")]
    pub input: String,
    /// Output files name without extension
    #[arg(short, long, default_value = "build/out")]
    pub output: String,
    /// Only build - don't run
    #[arg(short, long)]
    pub build: bool,
}

pub fn main() {
    let args = Args::parse();
    if let Err(e) = main_args(args) {
        println!("\n{e}");
    }
}

pub fn main_args(args: Args) -> Result<(), WError> {
    if let Some(path) = PathBuf::from(&args.output).parent() {
        if let Err(e) = fs::create_dir_all(path) {
            if !matches!(e.kind(), ErrorKind::AlreadyExists) {
                cprintln!("<r,bold>Failed to create directories for output files</>");
                panic!();
            }
        }
    }

    print!("Parsing... ");
    time!(
        let parsed = parse(PathBuf::from(&args.input)).unwrap();
    );

    print!("Resolving Names... ");
    time!(
        let (global_table, unprocessed_functions) = resolve(parsed)?;
    );

    print!("Compiling... ");
    time!(
        let assembly = compile(global_table, unprocessed_functions)?;
    );

    print!("Writing Assembly... ");
    time!(
        fs::write(PathBuf::from(format!("{}.asm", &args.output)), assembly.as_bytes()).unwrap();
    );


    print!("Assembling (NASM)... ");
    time!(
        assemble(&args.output).unwrap();
    );

    #[cfg(target_os = "linux")]
    {
        println!("Linking (gcc)... ");
        time!(
            link_gcc(&args.output).unwrap();
        );

        if args.build {
            println!("Skipping execution")
        } else {
            println!("Executing...");
            run(&args.output);
        }
    }

    cprintln!("<g,bold>Done!</>");
    Ok(())
}
