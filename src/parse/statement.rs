use super::super::tokenize::token::Token;
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::expression::ExpressionNode;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct DeclareStatement {
    pub identifier: Token,
    pub ty: Token,
}
impl DeclareStatement {
    /// parse and get declare_statement
    ///
    /// declare_statement := Token::Type Token::Identifier Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<()> {
        if let Some(token) = tokens.peek() {
            if let Token::Type(_) = token.get_token() {
                tokens.consume_type()?;
                tokens.consume_identifier()?;
                tokens.consume_semicolon()?;
            }
        }
        return Ok(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: ExpressionNode,
}
impl ExpressionStatement {
    /// parse and get expression_statement
    ///
    /// expression_statement := expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<()> {
        if let Some(token) = tokens.peek() {
            match token.get_token() {
                Token::Number(_) | Token::Identifier(_) => {
                    ExpressionNode::parse(tokens)?;
                    tokens.consume_semicolon()?;
                }
                _ => (),
            }
        }
        return Ok(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expression: ExpressionNode,
}
impl ReturnStatement {
    /// parse and get return_statement
    ///
    /// return_statement := Token::Return expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
        tokens.consume_return()?;
        let expression_node = ExpressionNode::parse(tokens);
        tokens.consume_semicolon()?;
        expression_node
    }
}

#[cfg(test)]
mod tests {

    use super::super::testutil::*;
    use super::*;

    #[test]
    fn pass() {
        let mut tokens = Tokens::new(vec![
            mtoken(Token::Return),
            mtoken(Token::Number(10)),
            mtoken(Token::Semicolon),
        ]);
        let actual = ReturnStatement::parse(&mut tokens).unwrap();
        let expect = num(10);
        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic(expected = "expect [Token::Semicolon]: Consume(Consume(Some(number: 10)))")]
    fn fail_without_return() {
        let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
        ReturnStatement::parse(&mut tokens).expect("expect [Token::Semicolon]");
    }
}
