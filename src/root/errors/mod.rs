#[cfg(debug_assertions)]
use std::backtrace::Backtrace;
use std::fmt::{Display, Formatter};

use color_print::cformat;

use crate::root::parser::location::Location;
use crate::root::parser::path_storage::PathStorage;
#[cfg(debug_assertions)]
use crate::root::DEBUG_ON_ERROR;

pub mod compiler_errors;
pub mod evaluable_errors;
pub mod name_resolver_errors;
pub mod parser_errors;

/// Universal error for Whython-8
#[derive(Debug)]
pub struct WErr {
    error: String,
    location: Option<Location>, // ! Important, don't do file reads unless necessary (i.e. Display)
}

impl WErr {
    /// Create a new error
    ///
    /// EXPENSIVE - Use only for irrecoverable errors!
    pub fn n(error: impl Display, location: Location) -> WErr {
        let w = WErr {
            error: format!("{error}"),
            location: Some(location),
        };
        #[cfg(debug_assertions)]
        if DEBUG_ON_ERROR {
            println!("WErr created:");
            println!("{}", Backtrace::capture());
        }
        w
    }

    /// Create a new error wrapped in `Err`
    ///
    /// EXPENSIVE - Use only for irrecoverable errors!
    pub fn ne<T>(error: impl Display, location: Location) -> Result<T, WErr> {
        let w = WErr {
            error: format!("{error}"),
            location: Some(location),
        };
        #[cfg(debug_assertions)]
        if DEBUG_ON_ERROR {
            println!("WErr created:");
            println!("{}", Backtrace::capture());
        }
        Err(w)
    }

    /// Create an error wrapped in `Err` with no location information. Use only if truly applicable e.g. no main
    pub fn locationless_e<T>(error: impl Display) -> Result<T, WErr> {
        Err(WErr {
            error: format!("{error}"),
            location: None,
        })
    }

    /// Create an error with no location information. Use only if truly applicable e.g. no main
    pub fn locationless(error: impl Display) -> WErr {
        WErr {
            error: format!("{error}"),
            location: None,
        }
    }

    pub fn with_context<'a>(&'a self, path_storage: &'a PathStorage) -> WErrContext {
        WErrContext {
            err: self,
            path_storage,
        }
    }

    fn fmt(&self, f: &mut Formatter<'_>, path_storage: &PathStorage) -> std::fmt::Result {
        let text = if let Some(location) = &self.location {
            cformat!(
                "<r,bold>Error:</>\n    {}\n{}\n",
                self.error,
                location.with_context(path_storage)
            )
        } else {
            cformat!("<r,bold>Error:</>\n    {}", self.error)
        };
        f.write_str(&text)
    }
}

pub struct WErrContext<'a> {
    err: &'a WErr,
    path_storage: &'a PathStorage,
}

impl<'a> Display for WErrContext<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.err.fmt(f, self.path_storage)
    }
}
