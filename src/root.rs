use name_resolver::resolve_names::resolve_names;
use crate::root::parser::parse::parse;
// use crate::root::assembler::assemble::generate_assembly;
// use crate::root::name_resolver::processor::process;
use crate::time;
use clap::Parser;
use color_print::cprintln;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use crate::root::name_resolver::resolve::resolve;

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

pub const POINTER_SIZE: usize = 8;

/// Compiler for Whython files (.why)
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Main input file
    #[arg(short, long, default_value = "main.why")]
    pub input: String,
    /// Output files name without extension
    /// Main input file
    #[arg(short, long, default_value = "build/out")]
    pub output: String,
    /// Only build - don't run
    #[arg(short, long)]
    pub build: bool,
}

pub fn main() {


    // assemble("build/out").unwrap();
    // link_gcc_experimental("build/out").unwrap();
    // run_wine_experimental("build/out").unwrap();
    // return;

    let args = Args::parse();
    let _ = main_args(args);
}

pub fn main_args(args: Args) {
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

    resolve(parsed);


    // print!("Compiling... ");
    // time!(generate_assembly(&args.output, functions););
    //
    // print!("Assembling (NASM)... ");
    // time!(if assemble(&args.output).is_err() {
    //     return Err(AnyError::Other);
    // });

    // #[cfg(target_os = "windows")]
    // {
    //     println!("Linking (MSVC - link.exe)... ");
    //     time!(if link(&args.output).is_err() {
    //         return Err(AnyError::Other);
    //     });
    //     if args.build {
    //         println!("Skipping execution")
    //     } else {
    //         println!("Executing... ");
    //         run(&args.output);
    //     }
    // }
    // #[cfg(target_os = "linux")]
    // {
    //     cprintln!("<yellow,bold>Compilation and execution on Linux may be buggy!</>");
    //     println!("Linking (gcc)... ");
    //     time!(
    //         let res = link_gcc_experimental(&args.output);
    //         if res.is_err() {
    //             return Err(AnyError::Other);
    //         }
    //     );
    //
    //     if args.build {
    //         println!("Skipping execution")
    //     } else {
    //         println!("Executing (wine)... ");
    //         if run_wine_experimental(&args.output).is_err() {
    //             return Err(AnyError::Other);
    //         }
    //     }
    // }

    cprintln!("<g,bold>Done!</>");
}
