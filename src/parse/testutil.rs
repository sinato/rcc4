use super::super::tokenize::token::{ManagedToken, Token};
use super::expression::{ExpressionNode, Operator};

pub fn mtoken(token: Token) -> ManagedToken {
    ManagedToken::new(token, 0, 0)
}

pub fn num(num: u64) -> Box<ExpressionNode> {
    ExpressionNode::create_single_node_num(num)
}

pub fn ide(ide: &str) -> Box<ExpressionNode> {
    ExpressionNode::create_single_node_ide(ide.to_owned())
}

pub fn exp(operator: &str, operand: Vec<Box<ExpressionNode>>) -> Box<ExpressionNode> {
    let operator = match operator {
        "+" => Operator::Add,
        "*" => Operator::Mul,
        "=" => Operator::Eq,
        _ => unimplemented!(),
    };
    Box::new(ExpressionNode { operator, operand })
}
