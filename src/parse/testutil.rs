use super::super::tokenize::token::{ManagedToken, Token};
use super::expression::ExpressionNode;

pub fn mtoken(token: Token) -> ManagedToken {
    ManagedToken::new(token, 0, 0)
}

pub fn num(num: u64) -> Box<ExpressionNode> {
    ExpressionNode::create_single_node(Token::Number(num))
}

pub fn ide(ide: &str) -> Box<ExpressionNode> {
    ExpressionNode::create_single_node(Token::Identifier(ide.to_owned()))
}

pub fn exp(operator: &str, operand: Vec<Box<ExpressionNode>>) -> Box<ExpressionNode> {
    let operator = Token::Operator(operator.to_owned());
    Box::new(ExpressionNode { operator, operand })
}
