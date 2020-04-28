pub mod node;

use super::tokenize::token::Token;
use node::Node;
use std::iter::{IntoIterator, Peekable};
use std::slice::IterMut;

pub fn parse(tokens: Vec<Token>) -> Box<Node> {
    get_sum(&mut tokens.clone())
}

fn get_sum(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut tokens_iter = tokens.into_iter().peekable();
    let mut nodes: Vec<Box<Node>> = Vec::new();
    while let Some(_) = tokens_iter.peek() {
        let mut target_tokens = consume_to_binary_operator(&mut tokens_iter, "+".to_owned());
        nodes.push(get_multi(&mut target_tokens));
    }
    if nodes.len() == 1 {
        nodes.get(0).unwrap().to_owned()
    } else {
        Box::new(Node {
            operator: Token::Operator("+".to_owned()),
            operand: nodes,
        })
    }
}

fn get_multi(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut tokens_iter = tokens.into_iter().peekable();
    let mut nodes: Vec<Box<Node>> = Vec::new();
    while let Some(_) = tokens_iter.peek() {
        let mut target_tokens = consume_to_binary_operator(&mut tokens_iter, "*".to_owned());
        nodes.push(get_number(&mut target_tokens));
    }
    if nodes.len() == 1 {
        nodes.get(0).unwrap().to_owned()
    } else {
        Box::new(Node {
            operator: Token::Operator("*".to_owned()),
            operand: nodes,
        })
    }
}

fn get_number(tokens: &mut Vec<Token>) -> Box<Node> {
    let mut tokens = tokens.into_iter().peekable();
    if tokens.len() != 1 {
        println!("panic get number {:?}", tokens);
        panic!("")
    } else {
        Node::create_single_node(tokens.next().unwrap().to_owned())
    }
}

fn consume_to_binary_operator(
    tokens: &mut Peekable<IterMut<Token>>,
    operator: String,
) -> Vec<Token> {
    let mut target_tokens: Vec<Token> = Vec::new();
    while let Some(token) = tokens.peek() {
        if let Token::Operator(op) = token {
            if op == &operator {
                tokens.next();
                break;
            }
        }
        target_tokens.push(tokens.next().unwrap().to_owned());
    }
    target_tokens
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn binary_add() {
        let actual: Node = *parse(vec![
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

    #[test]
    fn binary_add_mul() {
        let actual = *parse(vec![
            Token::Number(10),
            Token::Operator("+".to_owned()),
            Token::Number(20),
            Token::Operator("*".to_owned()),
            Token::Number(30),
            Token::Operator("+".to_owned()),
            Token::Number(40),
        ]);
        let expect: Node = Node {
            operator: Token::Operator("+".to_owned()),
            operand: vec![
                Node::create_single_node(Token::Number(10)),
                Box::new(Node {
                    operator: Token::Operator("*".to_owned()),
                    operand: vec![
                        Node::create_single_node(Token::Number(20)),
                        Node::create_single_node(Token::Number(30)),
                    ],
                }),
                Node::create_single_node(Token::Number(40)),
            ],
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn consume_to_binary_operator_add() {
        let tokens = &mut vec![
            Token::Number(10),
            Token::Number(20),
            Token::Operator("+".to_owned()),
        ];
        let mut tokens: Peekable<IterMut<Token>> = tokens.into_iter().peekable();
        let actual = consume_to_binary_operator(&mut tokens, "+".to_string());
        assert_eq!(actual, vec![Token::Number(10), Token::Number(20)]);
    }

    #[test]
    fn consume_to_binary_operator_eol() {
        let tokens = &mut vec![Token::Number(10), Token::Number(20)];
        let mut tokens: Peekable<IterMut<Token>> = tokens.into_iter().peekable();
        let actual = consume_to_binary_operator(&mut tokens, "+".to_string());
        assert_eq!(actual, vec![Token::Number(10), Token::Number(20)]);
    }
}
