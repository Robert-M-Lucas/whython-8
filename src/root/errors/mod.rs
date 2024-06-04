use std::fmt::{Display, Formatter};
use color_print::cformat;
use crate::root::parser::parse::Location;

pub mod parser_errors;
pub mod name_resolver_errors;

pub struct WError {
    error: String,
    location: Option<Location> // ! Important, don't do file reads unless necessary (i.e. Display)
}

impl WError {
    pub fn n(error: impl Display, location: Location) -> WError {
        WError {
            error: format!("{error}"),
            location: Some(location)
        }
    }

    pub fn locationless(error: impl Display) -> WError {
        WError {
            error: format!("{error}"),
            location: None
        }
    }
}

impl Display for WError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = if let Some(location) = &self.location {
            cformat!("<r,bold>Error:</>\n    {}\n<c,bold>At:</>\n{}", self.error, location)
        }
        else {
            cformat!("<r,bold>Error:</>\n    {}", self.error)
        };
        f.write_str(&text)
    }
}