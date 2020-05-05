use super::super::tokenize::token::{ManagedToken, Token};
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::expression::ExpressionNode;
use super::util::get_space;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Declare(DeclareStatement),
    Expression(ExpressionStatement),
}
impl Statement {
    /// parse and get statement if target statement exists
    ///
    /// statement := declare_statement | expression_statement
    pub fn parse(tokens: &mut Tokens) -> Result<Option<Statement>> {
        match tokens.peek() {
            Some(token) => match token.get_token() {
                Token::Type(_) => {
                    let declare_statement = DeclareStatement::parse(tokens)?;
                    Ok(Some(Statement::Declare(declare_statement)))
                }
                Token::Number(_) | Token::Identifier(_) => {
                    let expression_statement = ExpressionStatement::parse(tokens)?;
                    Ok(Some(Statement::Expression(expression_statement)))
                }
                _ => Ok(None),
            },
            None => Ok(None),
        }
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        match self {
            Statement::Declare(statement) => statement.to_string(tab_level),
            Statement::Expression(statement) => statement.to_string(tab_level),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclareStatement {
    pub identifier: ManagedToken,
    pub ty: ManagedToken,
}
impl DeclareStatement {
    /// parse and get declare_statement
    ///
    /// declare_statement := Token::Type Token::Identifier Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<DeclareStatement> {
        if let Some(token) = tokens.peek() {
            if let Token::Type(_) = token.get_token() {
                let ty = tokens.consume_type()?;
                let identifier = tokens.consume_identifier()?;
                tokens.consume_semicolon()?;
                return Ok(DeclareStatement { identifier, ty });
            }
        }
        Err(ParseError::Unexpect(tokens.next()))
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        format!(
            "{}declare_statement -> {}, {}\n",
            get_space(tab_level),
            self.identifier,
            self.ty
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression_node: ExpressionNode,
}
impl ExpressionStatement {
    /// parse and get expression_statement
    ///
    /// expression_statement := expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<ExpressionStatement> {
        if let Some(token) = tokens.peek() {
            match token.get_token() {
                Token::Number(_) | Token::Identifier(_) => {
                    let expression_node = *ExpressionNode::parse(tokens)?;
                    tokens.consume_semicolon()?;
                    return Ok(ExpressionStatement { expression_node });
                }
                _ => (),
            }
        }
        Err(ParseError::Unexpect(tokens.next()))
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        format!(
            "{}expression_statement ->\n{}",
            get_space(tab_level),
            self.expression_node.to_string(tab_level + 1)
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expression_node: ExpressionNode,
}
impl ReturnStatement {
    /// parse and get return_statement
    ///
    /// return_statement := Token::Return expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<ReturnStatement> {
        tokens.consume_return()?;
        let expression_node = *ExpressionNode::parse(tokens)?;
        tokens.consume_semicolon()?;
        Ok(ReturnStatement { expression_node })
    }
}

#[cfg(test)]
mod tests {

    use super::super::testutil::*;
    use super::*;

    #[cfg(test)]
    mod test_return_statement {
        use super::*;

        #[test]
        fn pass() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Return),
                mtoken(Token::Number(10)),
                mtoken(Token::Semicolon),
            ]);
            let actual = ReturnStatement::parse(&mut tokens).unwrap();
            let expect = ReturnStatement {
                expression_node: *num(10),
            };
            assert_eq!(actual, expect);
        }

        #[test]
        #[should_panic(expected = "expect [Token::Semicolon]: Consume(Consume(Some(number: 10)))")]
        fn fail_without_return() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
            ReturnStatement::parse(&mut tokens).expect("expect [Token::Semicolon]");
        }
    }
}
