#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    custom::install();
}