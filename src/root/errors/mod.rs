use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};
use color_print::cformat;
use crate::root::parser::parse::Location;
#[cfg(debug_assertions)]
use crate::root::DEBUG_ON_ERROR;

pub mod parser_errors;
pub mod name_resolver_errors;
pub mod evaluable_errors;

pub struct WErr {
    error: String,
    location: Option<Location> // ! Important, don't do file reads unless necessary (i.e. Display)
}

impl WErr {
    pub fn n(error: impl Display, location: Location) -> WErr {
        let w = WErr {
            error: format!("{error}"),
            location: Some(location)
        };
        #[cfg(debug_assertions)]
        if DEBUG_ON_ERROR {
            println!("{}", Backtrace::capture());
            println!("\n{w}");
        }
        w
    }

    pub fn ne<T>(error: impl Display, location: Location) -> Result<T, WErr> {
        let w = WErr {
            error: format!("{error}"),
            location: Some(location)
        };
        #[cfg(debug_assertions)]
        if DEBUG_ON_ERROR {
            println!("{}", Backtrace::capture());
            println!("\n{w}");
        }
        Err(w)
    }

    pub fn locationless<T>(error: impl Display) -> Result<T, WErr> {
        Err(WErr {
            error: format!("{error}"),
            location: None
        })
    }
}

impl Display for WErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = if let Some(location) = &self.location {
            cformat!("<r,bold>Error:</>\n    {}\n{}\n", self.error, location)
        }
        else {
            cformat!("<r,bold>Error:</>\n    {}", self.error)
        };
        f.write_str(&text)
    }
}