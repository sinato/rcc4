use super::super::tokenize::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub operator: Token,
    pub operand: Vec<Box<Node>>,
}

impl Node {
    pub fn get_operator_clone(&self) -> Token {
        self.operator.clone()
    }
    pub fn get_operand(self) -> Vec<Box<Node>> {
        self.operand
    }
    pub fn create_single_node(token: Token) -> Box<Node> {
        Box::new(Node {
            operator: token,
            operand: Vec::new(),
        })
    }
}
