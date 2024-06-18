use std::mem::MaybeUninit;
use std::hint::black_box;
use arr_macro::arr;
use pinned_init::{pin_data, pin_init};
use std::mem;

mod root;

#[pin_data]
struct Large {
    #[pin]
    inner: [u8; 10_000]
}

fn main() {
    root::main();
}
