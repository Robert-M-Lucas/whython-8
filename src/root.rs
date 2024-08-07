use crate::root::compiler::compile::compile;
use crate::root::errors::{WErr, WErrContext};
use crate::root::name_resolver::resolve::resolve;
use crate::root::parser::parse::parse;
use crate::root::runner::{assemble, link_gcc, run};
use crate::time;
use clap::Parser;
use color_print::cprintln;
use num_format::{Locale, ToFormattedString};
use shared::common::ByteSize;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::time::Instant;
use crate::root::parser::path_storage::PathStorage;

#[cfg(debug_assertions)]
pub const DEBUG_ON_ERROR: bool = false;

pub mod assembler;
pub mod builtin;
pub mod compiler;
pub mod errors;
pub mod name_resolver;
mod ob;
pub mod parser;
pub mod runner;
pub mod shared;
mod unrandom;
pub mod utils;

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

pub fn main_args(args: Args) -> Result<(), String> {
    if let Some(path) = PathBuf::from(&args.output).parent() {
        if let Err(e) = fs::create_dir_all(path) {
            if !matches!(e.kind(), ErrorKind::AlreadyExists) {
                cprintln!("<r,bold>Failed to create directories for output files</>");
                panic!();
            }
        }
    }

    
    print!("Parsing files... ");
    time!(
        let mut path_storage = PathStorage::new(&args.input).unwrap(); // TODO: 
        let toplevel_tokens = parse(&mut path_storage)
            .map_err(|e| e.with_context(&path_storage).to_string())?;
    );

    print!("Resolving Names... ");
    time!(
        let (global_table, unprocessed_functions) = resolve(toplevel_tokens)
        .map_err(|e| e.with_context(&path_storage).to_string())?;
    );

    print!("Compiling... ");
    time!(
        let assembly = compile(global_table, unprocessed_functions, &path_storage)
            .map_err(|e| e.with_context(&path_storage).to_string())?;
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
        print!("Linking (gcc)... ");

        let t = Instant::now();
        link_gcc(&args.output).unwrap();
        let end = t.elapsed();
        let size = File::open(format!("{}.out", args.output))
            .unwrap()
            .metadata()
            .unwrap()
            .len()
            .to_formatted_string(&Locale::en);

        cprintln!("<g,bold>Completed [{:?}] - {} bytes</>", end, size);

        if args.build {
            println!("Skipping execution")
        } else {
            let termsize::Size { rows: _, cols } = termsize::get().unwrap();
            const EXECUTING: &str = "Executing";
            if cols < EXECUTING.len() as u16 || cols > 300 {
                cprintln!("<s><b>Executing...</>");
            } else {
                let padl = (cols - EXECUTING.len() as u16) / 2;
                let padr = if ((cols - EXECUTING.len() as u16) % 2) == 1 {
                    padl + 1
                } else {
                    padl
                };
                cprintln!(
                    "<s><b>{}{}{}</>",
                    "-".repeat(padl as usize),
                    EXECUTING,
                    "-".repeat(padr as usize)
                );
            }
            run(&args.output);
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        println!("Not linking / executing as OS is not Linux");
    }

    cprintln!("<g,bold>Done!</>");
    Ok(())
}
