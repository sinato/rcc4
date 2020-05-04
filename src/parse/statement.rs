use super::super::tokenize::token::Token;
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::expression::parse_expression_node;
use super::node::ExpressionNode;

type Result<T> = std::result::Result<T, ParseError>;

/// parse and get declare_statement
///
/// declare_statement := Token::Type Token::Identifier Token::Semicolon
pub fn parse_declare_statement(tokens: &mut Tokens) -> Result<()> {
    if let Some(token) = tokens.peek() {
        if let Token::Type(_) = token.get_token() {
            tokens.consume_type()?;
            tokens.consume_identifier()?;
            tokens.consume_semicolon()?;
        }
    }
    return Ok(());
}

/// parse and get return_statement
///
/// return_statement := Token::Return expression_node Token::Semicolon
pub fn parse_return_statement(tokens: &mut Tokens) -> Result<Box<ExpressionNode>> {
    tokens.consume_return()?;
    let expression_node = parse_expression_node(tokens);
    tokens.consume_semicolon()?;
    expression_node
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
        let actual = parse_return_statement(&mut tokens).unwrap();
        let expect = num(10);
        assert_eq!(actual, expect);
    }

    #[test]
    #[should_panic(expected = "expect [Token::Semicolon]: Consume(Consume(Some(number: 10)))")]
    fn fail_without_return() {
        let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
        parse_return_statement(&mut tokens).expect("expect [Token::Semicolon]");
    }
}
