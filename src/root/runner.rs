use std::fs;
use std::process::Command;

use crate::ret_time;
use color_print::cprintln;

use crate::root::utils::try_run_program;

/// Assembles written assembly code
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

/// Links assembled code
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

/// Runs the built program
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

    let termsize::Size {rows, cols} = termsize::get().unwrap();
    const EXITED: &str = "Exited";
    if cols > EXITED.len() as u16 && cols < 300 {
        let padl = (cols - EXITED.len() as u16) / 2;
        let padr = if ((cols - EXITED.len() as u16) % 2) == 1 {
            padl + 1
        } else { padl };
        cprintln!("\n<s><r>{}{}{}</>", "-".repeat(padl as usize), EXITED, "-".repeat(padr as usize));
    }

    println!("\nExited with return code {}", code);
    cprintln!("<g,bold>Completed [{:?}]</>", time);
}
