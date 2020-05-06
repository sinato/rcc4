use super::super::tokenize::token::Token;
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::util::get_space;
use std::fmt;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Mul,
    Eq,
    FnCall(String),
    Identifier(String),
    Num(u64),
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "operator: +"),
            Operator::Mul => write!(f, "operator: *"),
            Operator::Eq => write!(f, "operator: ="),
            Operator::FnCall(fn_name) => write!(f, "function call: {}", fn_name),
            Operator::Identifier(identifier) => write!(f, "identifier: {}", identifier),
            Operator::Num(num) => write!(f, "number: {}", num),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionNode {
    pub operator: Operator,
    pub operand: Vec<Box<ExpressionNode>>,
}
impl ExpressionNode {
    /// parse and get expression_node
    ///
    /// expression_node := eq_node
    pub fn parse(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
        parse_eq_node(tokens)
    }
    pub fn get_operator_clone(&self) -> Operator {
        self.operator.clone()
    }
    pub fn get_operand(self) -> Vec<Box<ExpressionNode>> {
        self.operand
    }
    pub fn create_single_node_num(num: u64) -> Box<ExpressionNode> {
        Box::new(ExpressionNode {
            operator: Operator::Num(num),
            operand: vec![],
        })
    }
    pub fn create_single_node_ide(identifier: String) -> Box<ExpressionNode> {
        Box::new(ExpressionNode {
            operator: Operator::Identifier(identifier),
            operand: vec![],
        })
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        let mut s = "".to_owned();
        s += &format!("{}{}\n", get_space(tab_level), self.operator);
        for val in self.operand.iter() {
            s += &format!("{}", val.to_string(tab_level + 1));
        }
        s
    }
}

/// parse and get eq_node
///
/// eq_node := add_node (Token::Operator("=") add_node)*
fn parse_eq_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let mut operand = vec![parse_add_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("=") {
        tokens.next(); // consume "="
        operand.push(parse_add_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(Operator::Eq, operand))
}

/// parse and get add_node
///
/// add_node := mul_node (Token::Operator("+") mul_node)*
fn parse_add_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let mut operand = vec![parse_mul_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("+") {
        tokens.next(); // consume "+"
        operand.push(parse_mul_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(Operator::Add, operand))
}

/// parse and get mul_node
///
/// mul_node := fn_call_node (Token::Operator("*") fn_call_node)*
fn parse_mul_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    let mut operand = vec![parse_fn_call_node(tokens)?];

    while let Some(_token) = tokens.check_next_operator("*") {
        tokens.next(); // consume "*"
        operand.push(parse_fn_call_node(tokens)?);
    }
    Ok(reduce_redundunt_binary_operation(Operator::Mul, operand))
}

/// parse and get fn_call_node (function call node)
///
/// fn_call_node := Token::Identifier Token::Parenthesis("(") (expression_node (Token::Comma expresssion_node)*)? Token::Parenthesis(")") | leaf_node
fn parse_fn_call_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    // match Token::Identifier Token::Parenthesis
    if let Some(token) = tokens.peek() {
        if let Token::Identifier(_) = token.get_token() {
            if let Some(token2) = tokens.peek2() {
                if let Token::Parenthesis(_) = token2.get_token() {
                    let identifier = tokens.next().unwrap().get_token().get_identifier().unwrap();
                    tokens.next(); // consume (

                    // match (expression_node (Token::Comma expresssion_node)*)?
                    let mut parameters: Vec<Box<ExpressionNode>> = vec![];
                    if tokens.check_next_is_expression_node() {
                        parameters.push(ExpressionNode::parse(tokens)?);
                    }
                    loop {
                        if let Some(token) = tokens.peek() {
                            if let Token::Comma = token.get_token() {
                                tokens.next(); // consume ,
                                parameters.push(ExpressionNode::parse(tokens)?);
                                continue;
                            }
                        }
                        break;
                    }

                    tokens.next(); // consume )
                    return Ok(Box::new(ExpressionNode {
                        operator: Operator::FnCall(identifier),
                        operand: parameters,
                    }));
                }
            }
        }
    }
    parse_leaf_node(tokens)
}

/// parse and get leaf_node
///
/// leaf_node := Token::Number | Token::Identifier
fn parse_leaf_node(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    if let Some(token) = tokens.peek() {
        if let Token::Number(_) = token.get_token() {
            let num = tokens.next().unwrap().get_token().get_number().unwrap();
            return Ok(ExpressionNode::create_single_node_num(num));
        }
        if let Token::Identifier(_) = token.get_token() {
            let identifier = tokens.next().unwrap().get_token().get_identifier().unwrap();
            return Ok(ExpressionNode::create_single_node_ide(identifier));
        }
    }
    Err(ParseError::Unexpect(tokens.next()))
}

/// simplify redundunt tree
/// from "root - operator - [num]"
/// to "root - num"
fn reduce_redundunt_binary_operation(
    operator: Operator,
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

        use super::*;

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
        use super::*;

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
        use super::*;

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
    mod tests_parse_fn_call_node {
        use super::*;

        #[test]
        fn pass_leaf_node() {
            let mut tokens = Tokens::new(vec![ManagedToken::new(Token::Number(10), 0, 0)]);
            let actual = parse_fn_call_node(&mut tokens).unwrap();
            assert_eq!(actual, num(10));
        }

        #[test]
        fn pass_fn_call_node() {
            let mut tokens = Tokens::new(vec![
                ManagedToken::new(Token::Identifier("func".to_owned()), 0, 0),
                ManagedToken::new(Token::Parenthesis("(".to_owned()), 0, 0),
                ManagedToken::new(Token::Parenthesis(")".to_owned()), 0, 0),
            ]);
            let actual = *parse_fn_call_node(&mut tokens).unwrap();
            assert_eq!(
                actual,
                ExpressionNode {
                    operator: Operator::FnCall("func".to_owned()),
                    operand: vec![]
                }
            );
        }
    }

    #[cfg(test)]
    mod tests_parse_leaf_node {
        use super::*;

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
