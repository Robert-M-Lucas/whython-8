use std::fmt::{Display, Formatter};
use color_print::cformat;
use crate::root::parser::parse::Location;

pub mod parser_errors;
pub mod name_resolver_errors;
pub mod evaluable_errors;

pub struct WErr {
    error: String,
    location: Option<Location> // ! Important, don't do file reads unless necessary (i.e. Display)
}

impl WErr {
    pub fn n(error: impl Display, location: Location) -> WErr {
        WErr {
            error: format!("{error}"),
            location: Some(location)
        }
    }

    pub fn locationless(error: impl Display) -> WErr {
        WErr {
            error: format!("{error}"),
            location: None
        }
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