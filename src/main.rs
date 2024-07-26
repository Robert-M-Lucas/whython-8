use nom::character::complete::anychar;
use nom::{AsBytes, IResult};
use crate::root::parser::parse::ErrorTree;

mod root;


fn main() {
    root::main();
}
