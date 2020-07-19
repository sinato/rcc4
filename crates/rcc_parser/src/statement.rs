use super::error::ParseError;
use super::expression::Expression;
use super::util::get_space;
use rcc_syntax::token::{ManagedToken, Token};
use rcc_syntax::tokens::Tokens;

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
pub struct TypeStruct {
    base: String,
    pointer: u32,
    identifier: String,
    post: Vec<u32>,
}
impl TypeStruct {
    pub fn parse(tokens: &mut Tokens) -> Result<TypeStruct> {
        let base = tokens.consume_type()?.get_token().get_type()?;
        let pointer = 0; // TODO
        let identifier = tokens.consume_identifier()?.get_token().get_identifier()?;
        let post = vec![]; // TODO
        Ok(TypeStruct {
            base,
            pointer,
            identifier,
            post,
        })
    }
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        format!(
            "{}declare_statement -> type {}, identifier {}\n",
            get_space(tab_level),
            self.base,
            self.identifier
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclareStatement {
    pub type_struct: TypeStruct,
}
impl DeclareStatement {
    pub fn parse(tokens: &mut Tokens) -> Result<DeclareStatement> {
        let type_struct = TypeStruct::parse(tokens)?;
        tokens.consume_semicolon()?;
        return Ok(DeclareStatement { type_struct });
    }
    pub fn to_string(&self, tab_level: u32) -> String {
        format!(
            "{}declare_statement -> {}\n",
            get_space(tab_level),
            self.type_struct.to_string(tab_level),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}
impl ExpressionStatement {
    /// parse and get expression_statement
    ///
    /// expression_statement := expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<ExpressionStatement> {
        if let Some(token) = tokens.peek() {
            match token.get_token() {
                Token::Number(_) | Token::Identifier(_) => {
                    let expression = Expression::parse(tokens)?;
                    tokens.consume_semicolon()?;
                    return Ok(ExpressionStatement { expression });
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
            self.expression.to_string(tab_level + 1)
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expression: Expression,
}
impl ReturnStatement {
    /// parse and get return_statement
    ///
    /// return_statement := Token::Return expression_node Token::Semicolon
    pub fn parse(tokens: &mut Tokens) -> Result<ReturnStatement> {
        tokens.consume_return()?;
        let expression = Expression::parse(tokens)?;
        tokens.consume_semicolon()?;
        Ok(ReturnStatement { expression })
    }
}

#[cfg(test)]
mod tests {

    use super::super::testutil::*;
    use super::*;

    #[cfg(test)]
    mod test_return_statement {
        use super::*;

        /*
        #[test]
        fn pass() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Return),
                mtoken(Token::Number(10)),
                mtoken(Token::Semicolon),
            ]);
            let actual = ReturnStatement::parse(&mut tokens).unwrap();
            let expect = ReturnStatement {
                expression: *num(10),
            };
            assert_eq!(actual, expect);
        }
        */

        #[test]
        #[should_panic(expected = "expect [Token::Semicolon]: Consume(Consume(Some(number: 10)))")]
        fn fail_without_return() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Number(10)), mtoken(Token::Semicolon)]);
            ReturnStatement::parse(&mut tokens).expect("expect [Token::Semicolon]");
        }
    }
}
