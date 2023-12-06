use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use pest::pratt_parser::{PrattParser, Assoc, Op};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use pest::Parser;
use lazy_static::lazy_static;

use crate::frame::Frame;
use crate::expr::{LetBinding, BinaryExpr, Expr, Variable, CondExpr};
use crate::bin_op::{self, BinaryOp};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct PairParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = PrattParser::new()
        .op(Op::infix(Rule::Eq, Assoc::Left) | Op::infix(Rule::Ne, Assoc::Left))
        .op(Op::infix(Rule::Gt, Assoc::Left) | Op::infix(Rule::Lt, Assoc::Left) | Op::infix(Rule::Ge, Assoc::Left) | Op::infix(Rule::Le, Assoc::Left))
        .op(Op::infix(Rule::Add, Assoc::Left) | Op::infix(Rule::Sub, Assoc::Left))
        .op(Op::infix(Rule::Mul, Assoc::Left) | Op::infix(Rule::Div, Assoc::Left))
        ;

    static ref RULE_OP_MAP: BTreeMap<Rule, BinaryOp> = BTreeMap::from([
        (Rule::Add, bin_op::add as BinaryOp),
        (Rule::Sub, bin_op::sub as BinaryOp),
        (Rule::Mul, bin_op::mul as BinaryOp),
        (Rule::Div, bin_op::div as BinaryOp),
        (Rule::Mod, bin_op::mod_ as BinaryOp),
        (Rule::Eq, bin_op::eq as BinaryOp),
        (Rule::Ne, bin_op::ne as BinaryOp),
        (Rule::Gt, bin_op::gt as BinaryOp),
        (Rule::Lt, bin_op::lt as BinaryOp),
        (Rule::Ge, bin_op::ge as BinaryOp),
        (Rule::Le, bin_op::le as BinaryOp),
    ]);
}

fn pratt_parser() -> &'static PrattParser<Rule> {
    &*PRATT_PARSER
}

struct ParserContext {
    scope: Rc<RefCell<Frame>>
}

impl ParserContext {
    fn new() -> Self {
        Self {
            scope: Rc::new(RefCell::new(Frame::new()))
        }
    }

    fn make_inner_scope(&mut self) {
        let new_scope = Frame::from_parent(self.scope.clone());
        self.scope = Rc::new(RefCell::new(new_scope));
    }
}

struct AstParser {
    context: ParserContext,
}

impl AstParser {
    fn new() -> Self {
        Self {
            context: ParserContext::new()
        }
    }
}

impl AstParser {
    fn parse_let_bind(&mut self, pairs: &mut Pairs<Rule>) -> LetBinding {
        self.context.make_inner_scope();
        LetBinding {
            identifier: pairs.next().unwrap().as_str().to_string(),
            bind_expr: self.parse_expr(pairs.next().unwrap()),
            in_expr: self.parse_expr(pairs.next().unwrap()),
        }
    }

    fn parse_invocation(&mut self, pairs: Pairs<Rule>) -> Box<dyn Expr> {
        todo!()
    }

    fn parse_primary(&mut self, pair: Pair<Rule>) -> Box<dyn Expr> {
        match pair.as_rule() {
            Rule::Integer => Box::new(pair.as_str().parse::<i64>().unwrap()),
            Rule::Identifier => Box::new(Variable {
                index: pair.as_str().into(),
            }),
            Rule::BinExpr | Rule::LetBind | Rule::Invocation => self.parse_expr(pair),
            _ => unimplemented!(),
        }
    }

    fn parse_binary_expr(&mut self, pair: Pair<Rule>) -> Box<dyn Expr> {
        pratt_parser()
            .map_primary(|primary| match primary.as_rule() {
                Rule::Invocation => unimplemented!(),
                _ => self.parse_primary(primary),
            })
            .map_infix(|lhs, op, rhs| {
                Box::new(BinaryExpr {
                    lhs,
                    op: RULE_OP_MAP[&op.as_rule()],
                    rhs,
                })
            })
            .parse(pair.into_inner())
    }

    fn parse_cond_expr(&mut self, pairs: Pairs<Rule>) -> CondExpr {
        CondExpr {
            inner: pairs.map(|pair| self.parse_expr(pair)).collect()
        }
    }

    pub fn parse_expr(&mut self, pair: Pair<Rule>) -> Box<dyn Expr> {
        match pair.as_rule() {
            Rule::LetBind => Box::new(self.parse_let_bind(&mut pair.into_inner())),
            Rule::BinExpr => self.parse_binary_expr(pair),
            Rule::CondExpr => Box::new(self.parse_cond_expr(pair.into_inner())),
            Rule::Invocation => unimplemented!(),
            _ => self.parse_primary(pair),
        }
    }
}

pub fn parse() {
    let frame = Rc::new(RefCell::new(Frame::new()));
    let mut parser = AstParser::new();
    let p = PairParser::parse(Rule::Input, "
        let a = 1 in 
            let b = a in
                10 > a
            end
        end
    ").into_iter().next().unwrap().next().unwrap();
    let expr = parser.parse_expr(p);
    println!("{:?}", expr.evaluate(frame));
}

