pub mod identify_first_last;

use color_print::cprintln;
use std::io;
use std::io::ErrorKind;
use std::ops::{Add, Rem, Sub};
use std::process::ExitStatus;

/// Tries to run an executable showing a suitable message upon failure
pub fn try_run_program(name: &str, exit_status: io::Result<ExitStatus>) -> Result<ExitStatus, ()> {
    match exit_status {
        Ok(e) => Ok(e),
        Err(e) => {
            if matches!(e.kind(), ErrorKind::NotFound) {
                println!("Program `{name}` not found. Check to make sure it is in your path");
            } else {
                println!("Running `{name}` failed with error:\n{e}")
            }
            Err(())
        }
    }
}

/// Times how long code takes to execute and prints it
#[macro_export]
macro_rules! time {
    ($($tts:tt)*) => {
        let t = std::time::Instant::now();
        $($tts)*
        let end = t.elapsed();
        color_print::cprintln!("<g,bold>Completed [{:?}]</>", end);
    };
}

/// Times how long code takes to execute and puts it in `out`
#[macro_export]
macro_rules! ret_time {
    ($out: expr, $($tts:tt)*) => {
        let t = std::time::Instant::now();
        $($tts)*
        #[allow(clippy::needless_late_init)]
        {
            $out = t.elapsed();
        }
    };
}

/// Aligns a number to the next multiple of `alignment`
#[allow(dead_code)]
pub fn align<T: Copy + Sub<Output = T> + Rem<Output = T> + Add<Output = T>>(
    num: T,
    alignment: T,
) -> T {
    num + (alignment - (num % alignment)) % alignment
}

/// Prints a warning
pub fn warn(msg: &str) {
    cprintln!("\n<y,bold>Warning:</> <y>{}</>", msg);
}
