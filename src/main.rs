use crate::root::parser::parse::ErrorTree;
use nom::character::complete::anychar;
use nom::{AsBytes, IResult};

mod root;

fn main() {
    root::main();
}
