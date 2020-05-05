use super::super::tokenize::token::Token;
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::node::ExpressionNode;

type Result<T> = std::result::Result<T, ParseError>;

/// parse and get expression_node
///
/// expression_node := eq_node
pub fn parse_expression_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    parse_eq_node(tokens)
}

/// parse and get eq_node
///
/// eq_node := add_node (Token::Operator("=") add_node)*
fn parse_eq_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let operator = Token::Operator("=".to_owned());
    let mut operand = vec![parse_add_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("=") {
        tokens.next(); // consume "="
        operand.push(parse_add_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(operator, operand))
}

/// parse and get add_node
///
/// add_node := mul_node (Token::Operator("+") mul_node)*
fn parse_add_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let operator = Token::Operator("+".to_owned());
    let mut operand = vec![parse_mul_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("+") {
        tokens.next(); // consume "+"
        operand.push(parse_mul_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(operator, operand))
}

/// parse and get mul_node
///
/// mul_node := leaf_node (Token::Operator("*") leaf_node)*
fn parse_mul_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let operator = Token::Operator("*".to_owned());
    let mut operand = vec![parse_leaf_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("*") {
        tokens.next(); // consume "*"
        operand.push(parse_leaf_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(operator, operand))
}

/// parse and get leaf_node
///
/// leaf_node := Token::Number | Token::Identifier
fn parse_leaf_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    if let Some(token) = tokens.peek() {
        if let Token::Number(_) = token.get_token() {
            return Ok(ExpressionNode::create_single_node(
                tokens.next().unwrap().get_token().to_owned(),
            ));
        }
        if let Token::Identifier(_) = token.get_token() {
            return Ok(ExpressionNode::create_single_node(
                tokens.next().unwrap().get_token().to_owned(),
            ));
        }
    }
    Err(ParseError::Unexpect(tokens.next()))
}

/// simplify redundunt tree
/// from "root - operator - [num]"
/// to "root - num"
fn reduce_redundunt_binary_operation(
    operator: Token,
    operand: Vec<Box<ExpressionNode>>,
) -> Box<ExpressionNode> {
    if operand.len() == 1 {
        operand.get(0).unwrap().to_owned()
    } else {
        Box::new(ExpressionNode { operator, operand })
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::tokenize::token::ManagedToken;
    use super::super::testutil::*;
    use super::*;

    #[cfg(test)]
    mod tests_parse_eq_node {
        use self::super::*;

        #[test]
        fn pass_one_term() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10))]);
            let actual = parse_eq_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        #[should_panic(expected = "expect [Token::Number]: Unexpect(Some(semicolon))")]
        fn fail() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Semicolon)]);
            parse_eq_node(&mut tokens).expect("expect [Token::Number]");
        }

        #[test]
        fn pass_one_term_with_semicolon() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
            let actual = parse_eq_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        fn pass_two_terms() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Identifier("a".to_owned())),
                mtoken(Token::Operator("=".to_owned())),
                mtoken(Token::Number(10)),
                mtoken(Token::Operator("+".to_owned())),
                mtoken(Token::Number(20)),
            ]);
            let actual = parse_eq_node(&mut tokens).unwrap();
            let expect = exp("=", vec![ide("a"), exp("+", vec![num(10), num(20)])]);
            assert_eq!(actual, expect);
        }
    }

    #[cfg(test)]
    mod tests_parse_add_node {
        use self::super::*;

        #[test]
        fn pass_one_term() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10))]);
            let actual = parse_add_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        #[should_panic(expected = "expect [Token::Number]: Unexpect(Some(semicolon))")]
        fn fail() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Semicolon)]);
            parse_add_node(&mut tokens).expect("expect [Token::Number]");
        }

        #[test]
        fn pass_one_term_with_semicolon() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
            let actual = parse_add_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        fn pass_two_terms() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Number(10)),
                mtoken(Token::Operator("+".to_owned())),
                mtoken(Token::Number(20)),
                mtoken(Token::Operator("*".to_owned())),
                mtoken(Token::Number(30)),
            ]);
            let actual = parse_add_node(&mut tokens).unwrap();
            let expect = exp("+", vec![num(10), exp("*", vec![num(20), num(30)])]);
            assert_eq!(actual, expect);
        }
    }

    #[cfg(test)]
    mod tests_parse_mul_node {
        use self::super::*;

        #[test]
        fn pass_one_term() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10))]);
            let actual = parse_mul_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        #[should_panic(expected = "expect [Token::Number]: Unexpect(Some(semicolon))")]
        fn fail() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Semicolon)]);
            parse_mul_node(&mut tokens).expect("expect [Token::Number]");
        }

        #[test]
        fn pass_one_term_with_semicolon() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
            let actual = parse_mul_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        fn pass_two_terms() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Number(10)),
                mtoken(Token::Operator("*".to_owned())),
                mtoken(Token::Number(20)),
            ]);
            let actual = parse_mul_node(&mut tokens).unwrap();
            let expect = exp("*", vec![num(10), num(20)]);
            assert_eq!(actual, expect);
        }
    }

    #[cfg(test)]
    mod tests_parse_leaf_node {
        use self::super::*;

        #[test]
        fn pass() {
            let mut tokens = Tokens::new(vec![ManagedToken::new(Token::Number(10), 0, 0)]);
            let actual = parse_leaf_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        #[should_panic(expected = "expect [Token::Number]: Unexpect(Some(semicolon))")]
        fn fail() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Semicolon)]);
            parse_leaf_node(&mut tokens).expect("expect [Token::Number]");
        }
    }
}
