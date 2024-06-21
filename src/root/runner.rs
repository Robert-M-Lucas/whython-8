use std::fs;
use std::process::Command;

use color_print::cprintln;
use crate::ret_time;

use crate::root::utils::try_run_program;

pub fn run(output: &str) {
    let time;
    ret_time!(time,
        let full = fs::canonicalize(format!("{output}.out")).unwrap();
        let code = match Command::new(full).status() {
            Ok(r) => {
                match r.code() {
                    Some(c) => c,
                    None => {
                        cprintln!("<r,bold>\nProcess did not return an exit code. \
                        This could be due to a forceful termination</>");
                        return;
                    }
                }
            }
            Err(e) => {
                cprintln!("<r,bold>Starting process failed with error:\n{}</>", e);
                return;
            }
        };
    );

    // ? Here to circumvent some timing issues
    println!("\nExited with return code {}", code);
    cprintln!("<g,bold>Completed [{:?}]</>", time);
}

pub fn assemble(output: &str) -> Result<(), ()> {
    if !try_run_program(
        "nasm",
        Command::new("nasm")
            .args(["-f", "elf64", format!("{output}.asm").as_str()])
            .status(),
    )?
    .success()
    {
        cprintln!("<r,bold>NASM assembler step failed</>");
        return Err(());
    }
    Ok(())
}


pub fn link_gcc(output: &str) -> Result<(), ()> {
    if !try_run_program(
        "gcc",
        Command::new("gcc")
            .args([
                format!("{output}.o").as_str(),
                "-o",
                format!("{output}.out").as_str(),
            ])
            .status(),
    )?
    .success()
    {
        cprintln!("<r,bold>gcc linking step failed</>");
        return Err(());
    }

    Ok(())
}
