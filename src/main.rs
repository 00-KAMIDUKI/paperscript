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
    let args = std::env::args();
    let input = if args.len() == 1 {
        std::io::read_to_string(std::io::stdin()).unwrap()
    } else {
        let args: Vec<_> = args.collect();
        std::fs::read_to_string(args[1].as_str()).unwrap()
    };
    let expr = parser::parse(&input).unwrap();
    let runtime = runtime::Runtime::new(expr);
    println!("{:?}", runtime.evaluate().unwrap());
}

