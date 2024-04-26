use std::io;
use std::io::ErrorKind;
use std::ops::{Add, Rem, Sub};
use std::process::ExitStatus;

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

#[macro_export]
macro_rules! time {
    ($($tts:tt)*) => {
        let t = std::time::Instant::now();
        $($tts)*
        let end = t.elapsed();
        color_print::cprintln!("<g,bold>Completed [{:?}]</>", end);
    };
}

#[macro_export]
macro_rules! ret_time {
    ($out: expr, $($tts:tt)*) => {
        let t = std::time::Instant::now();
        $($tts)*
        $out = t.elapsed();
    };
}

pub fn align<T: Copy + Sub<Output = T> + Rem<Output = T> + Add<Output = T>>(
    num: T,
    alignment: T,
) -> T {
    num + (alignment - (num % alignment)) % alignment
}
