use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use pest::pratt_parser::{PrattParser, Assoc, Op};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use pest::Parser;
use lazy_static::lazy_static;

use crate::scope::Scope;
use crate::Expr;
use crate::expr::{LetBinding, BinaryExpr};
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
    scope: Rc<RefCell<Scope>>
}

impl ParserContext {
    fn new() -> Self {
        Self {
            scope: Rc::new(RefCell::new(Scope::new()))
        }
    }

    fn make_inner_scope(&mut self) {
        let new_scope = Scope::from_parent(self.scope.clone());
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
            name: pairs.next().unwrap().to_string(),
            bind_expr: self.parse_expr(pairs.next().unwrap()),
            in_expr: self.parse_expr(pairs.next().unwrap()),
            scope: self.context.scope.clone(),
        }
    }

    fn parse_invocation(&mut self, pair: Pairs<Rule>) -> Rc<dyn Expr> {
        unimplemented!()
    }

    fn parse_primary(&mut self, pair: Pair<Rule>) -> Rc<dyn Expr> {
        match pair.as_rule() {
            Rule::Integer => Rc::new(pair.as_str().parse::<i64>().unwrap()),
            Rule::Identifier => self.context.scope.borrow().find(&crate::VariableIndex{ name: pair.as_str().to_string() }).unwrap(),
            _ => unimplemented!(),
        }
    }

    fn parse_binary_expr(&mut self, pair: Pair<Rule>) -> Rc<dyn Expr> {
        pratt_parser()
            .map_primary(|primary| match primary.as_rule() {
                Rule::Invocation => unimplemented!(),
                _ => self.parse_primary(primary),
            })
            .map_infix(|lhs, op, rhs| {
                Rc::new(BinaryExpr {
                    lhs,
                    op: RULE_OP_MAP[&op.as_rule()],
                    rhs,
                })
            })
            .parse(pair.into_inner())
    }

    pub fn parse_expr(&mut self, pair: Pair<Rule>) -> Rc<dyn Expr> {
        match pair.as_rule() {
            Rule::LetBind => Rc::new(self.parse_let_bind(&mut pair.into_inner())),
            Rule::BinExpr => self.parse_binary_expr(pair),
            Rule::Invocation => unimplemented!(),
            _ => self.parse_primary(pair),
        }
    }
}

pub fn parse() {
    let mut parser = AstParser::new();
    let p = PairParser::parse(Rule::Input, "
        let a = 1 in
        let b = 2 in
        a + b
    ").into_iter().next().unwrap().next().unwrap();
    let expr = parser.parse_expr(p);
    println!("{:?}", expr.evaluate());
}

