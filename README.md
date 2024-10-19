# Whython-8

Whython-8 is a compiled programming language with no runtime. It compiles to raw NASM 
assembly without an intermediate step like LLVM. As this is a work-in-progress there
is no documentation on how to write Whython code however you should be able to get a
general idea by looking at any `.why` files e.g. in the root, in `std`, and in `reference`.
Some of these may be broken due to breaking changes

This is designed to work on Linux using `nasm` and `gcc` requiring a library
with `free`, `malloc`, and `printf` to be present (should be by default, 
all may not be required if not used in your code). This may work in WSL.

The `master` branch should now contain a semi-stable version whereas
the latest version can be found on `dev`. I also use `dev` to sync
my work so expect constant breaking changes on that branch.

## Usage

`cargo run` will compile and run `main.why`. By default this runs an example
using the `LinkedList` defined in `std/linked_list.why`.

Use `cargo run -- -i error.why` and `cargo run -- -i error2.why` to see
two examples of the rich error reporting in Whython

Use `cargo run -- [args]` to pass arguments to Whython

Use `cargo run -- -h` to get help

The binary for use without `cargo` commands can be found in `target/release` after running `cargo build -r`
