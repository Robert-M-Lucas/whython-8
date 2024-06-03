use std::{fs, thread};
use std::process::Command;
use std::time::Duration;

use color_print::cprintln;

use crate::ret_time;
use crate::root::utils::try_run_program;

#[cfg(target_os = "linux")]
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
    thread::sleep(Duration::from_millis(100));
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


#[cfg(target_os = "linux")]
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
