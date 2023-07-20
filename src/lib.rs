#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
    unused_parens,
	unused_macros,
	unused_variables,
	unused_assignments,
	non_upper_case_globals,
	non_snake_case,
	dead_code,
    clippy::borrow_interior_mutable_const
)]

mod frame;
mod moves;
mod status;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    frame::install();
	moves::install();
	status::install();
}