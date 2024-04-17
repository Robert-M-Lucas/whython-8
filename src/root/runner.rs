use std::{fs, thread};
use std::process::Command;
use std::time::Duration;

use color_print::cprintln;

use crate::ret_time;
use crate::root::utils::try_run_program;

#[cfg(target_os = "windows")]
pub fn run(output: &str) {
    let time;
    ret_time!(time,
        let full = fs::canonicalize(format!("{output}.exe")).unwrap();
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

#[cfg(target_os = "linux")]
pub fn run_wine_experimental(output: &str) -> Result<(), ()> {
    let time;
    ret_time!(time,
        let full = fs::canonicalize(format!("{output}.exe")).unwrap();
        let code = try_run_program("wine", Command::new("wine").args([full]).status())?
            .code()
            .unwrap();
    );

    println!("\nExited with return code {}", code);
    cprintln!("<g,bold>Completed [{:?}]</>", time);
    Ok(())
}

pub fn assemble(output: &str) -> Result<(), ()> {
    if !try_run_program(
        "nasm",
        Command::new("nasm")
            .args(["-f", "win64", format!("{output}.asm").as_str()])
            .status(),
    )?
    .success()
    {
        cprintln!("<r,bold>NASM assembler step failed</>");
        return Err(());
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn link(output: &str) -> Result<(), ()> {
    if !try_run_program(
        "link",
        Command::new("link")
            .args([
                // "/entry:main",
                format!("/out:{output}.exe").as_str(),
                "/SUBSYSTEM:CONSOLE",
                // "/LARGEADDRESSAWARE:NO",
                format!("{output}.obj").as_str(),
                ".\\libs\\kernel32.lib",
                ".\\libs\\msvcrt.lib",
                ".\\libs\\legacy_stdio_definitions.lib",
                ".\\libs\\legacy_stdio_wide_specifiers.lib",
                ".\\libs\\vcruntime.lib",
                ".\\libs\\ucrt.lib",
            ])
            .status(),
    )?
    .success()
    {
        cprintln!("<r,bold>MSVC linking step failed</>");
        return Err(());
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn link_gcc_experimental(output: &str) -> Result<(), ()> {
    if !try_run_program(
        "x86_64-w64-mingw32-gcc",
        Command::new("x86_64-w64-mingw32-gcc")
            .args([
                format!("{output}.obj").as_str(),
                "./libs/kernel32.lib",
                "-o",
                format!("{output}.exe").as_str(),
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
