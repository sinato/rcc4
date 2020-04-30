use super::super::tokenize::token::Token;
use super::super::tokenize::tokens::Tokens;
use super::node::ExpressionNode;

pub fn parse_expression(tokens: &mut Tokens) -> Box<ExpressionNode> {
    get_sum(tokens)
}

fn get_sum(tokens: &mut Tokens) -> Box<ExpressionNode> {
    let mut nodes: Vec<Box<ExpressionNode>> = Vec::new();
    while let Some(_) = tokens.peek() {
        let target_tokens = tokens.consume_to_binary_operator("+".to_owned());
        nodes.push(get_multi(&mut Tokens::new(target_tokens)));
    }
    if nodes.len() == 1 {
        nodes.get(0).unwrap().to_owned()
    } else {
        Box::new(ExpressionNode {
            operator: Token::Operator("+".to_owned()),
            operand: nodes,
        })
    }
}

fn get_multi(tokens: &mut Tokens) -> Box<ExpressionNode> {
    let mut nodes: Vec<Box<ExpressionNode>> = Vec::new();
    while let Some(_) = tokens.peek() {
        let target_tokens = tokens.consume_to_binary_operator("*".to_owned());
        nodes.push(get_number(&mut Tokens::new(target_tokens)));
    }
    if nodes.len() == 1 {
        nodes.get(0).unwrap().to_owned()
    } else {
        Box::new(ExpressionNode {
            operator: Token::Operator("*".to_owned()),
            operand: nodes,
        })
    }
}

fn get_number(tokens: &mut Tokens) -> Box<ExpressionNode> {
    if tokens.len() != 1 {
        println!("panic get number {:?}", tokens);
        panic!("")
    } else {
        ExpressionNode::create_single_node(tokens.next().unwrap().get_token().to_owned())
    }
}

#[cfg(test)]
mod tests {

    use self::super::super::super::tokenize::token::ManagedToken;
    use self::super::*;

    #[test]
    fn binary_add() {
        let mut tokens = Tokens::new(
            vec![
                Token::Number(10),
                Token::Operator("+".to_owned()),
                Token::Number(20),
            ]
            .into_iter()
            .map(|token| ManagedToken::new(token, 0, 0))
            .collect(),
        );
        let actual: ExpressionNode = *parse_expression(&mut tokens);
        let expect: ExpressionNode = ExpressionNode {
            operator: Token::Operator("+".to_owned()),
            operand: vec![
                ExpressionNode::create_single_node(Token::Number(10)),
                ExpressionNode::create_single_node(Token::Number(20)),
            ],
        };
        assert_eq!(actual, expect);
    }

    #[test]
    fn binary_add_mul() {
        let mut tokens = Tokens::new(
            vec![
                Token::Number(10),
                Token::Operator("+".to_owned()),
                Token::Number(20),
                Token::Operator("*".to_owned()),
                Token::Number(30),
                Token::Operator("+".to_owned()),
                Token::Number(40),
            ]
            .into_iter()
            .map(|token| ManagedToken::new(token, 0, 0))
            .collect(),
        );
        let actual: ExpressionNode = *parse_expression(&mut tokens);
        let expect: ExpressionNode = ExpressionNode {
            operator: Token::Operator("+".to_owned()),
            operand: vec![
                ExpressionNode::create_single_node(Token::Number(10)),
                Box::new(ExpressionNode {
                    operator: Token::Operator("*".to_owned()),
                    operand: vec![
                        ExpressionNode::create_single_node(Token::Number(20)),
                        ExpressionNode::create_single_node(Token::Number(30)),
                    ],
                }),
                ExpressionNode::create_single_node(Token::Number(40)),
            ],
        };
        assert_eq!(actual, expect);
    }
}
