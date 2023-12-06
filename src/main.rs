#![feature(fn_traits, map_try_insert, trait_upcasting, min_specialization)]

use frame::Frame;

mod type_;
mod value;
mod expr;
mod bin_op;
mod parser;
mod frame;
mod error;
mod runtime;


fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    let expr = parser::parse(&input).unwrap();
    let runtime = runtime::Runtime::new(expr);
    println!("{:?}", runtime.evaluate());
}

