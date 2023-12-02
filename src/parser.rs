use pest::pratt_parser::{PrattParser, Assoc, Op};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use pest::Parser;
use lazy_static::lazy_static;

use crate::Expr;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct PaperParser;

fn pratt_parser() -> &'static PrattParser<Rule> {
    lazy_static! {
        static ref PRATT_PARSER: PrattParser<Rule> = PrattParser::new()
            .op(Op::infix(Rule::Eq, Assoc::Left) | Op::infix(Rule::Ne, Assoc::Left))
            .op(Op::infix(Rule::Gt, Assoc::Left) | Op::infix(Rule::Lt, Assoc::Left) | Op::infix(Rule::Ge, Assoc::Left) | Op::infix(Rule::Le, Assoc::Left))
            .op(Op::infix(Rule::Add, Assoc::Left) | Op::infix(Rule::Sub, Assoc::Left))
            .op(Op::infix(Rule::Mul, Assoc::Left) | Op::infix(Rule::Div, Assoc::Left))
            ;
    }
    &*PRATT_PARSER
}

// fn parse_let_bind()

fn parse_let_bind(pairs: Pairs<Rule>) -> crate::expr::LetBinding {
    // pairs.next().unwrap().as_str();
}

fn parse_expr(pair: Pair<Rule>) -> Box<dyn Expr> {
    match pair.as_rule() {
        Rule::LetBind => Box::new(parse_let_bind(pair.into_inner())),
        Rule::BinExpr => unimplemented!(),
        _ => unreachable!(),
    }
}

pub fn parse() {
    let p = PaperParser::parse(Rule::Input, "
        let a = 1 in
        let b = 2 in
        a + b
    ").into_iter().next().unwrap();
    println!("{:?}", p);
}

