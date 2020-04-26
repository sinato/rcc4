pub mod node;

use super::tokenize::token::Token;
use node::Node;

pub fn parse(tokens: Vec<Token>) -> Node {
    if tokens.len() == 1 {
        Node {
            operator: tokens.get(0).unwrap().clone(),
            operand: Vec::new(),
        }
    } else {
        let mut operators: Vec<Token> = Vec::new();
        let mut operands: Vec<Box<Node>> = Vec::new();
        for token in tokens.into_iter() {
            match token {
                Token::Operator(_) => operators.push(token),
                Token::Number(_) => operands.push(Node::create_single_node(token)),
            }
        }
        Node {
            operator: operators.get(0).unwrap().to_owned(),
            operand: operands,
        }
    }
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn binary_add() {
        let actual: Node = parse(vec![
            Token::Number(10),
            Token::Operator("+".to_owned()),
            Token::Number(20),
        ]);
        let expect: Node = Node {
            operator: Token::Operator("+".to_owned()),
            operand: vec![
                Node::create_single_node(Token::Number(10)),
                Node::create_single_node(Token::Number(20)),
            ],
        };
        assert_eq!(actual, expect);
    }
}
