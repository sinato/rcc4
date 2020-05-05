use super::super::tokenize::token::ManagedToken;
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::statement::{ReturnStatement, Statement};
use super::util::get_space;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub identifier: ManagedToken,
    pub return_type: ManagedToken,
    pub argument_types: Vec<ManagedToken>,
    pub block: Vec<Statement>,
    pub return_statement: ReturnStatement,
}
impl Function {
    pub fn parse(tokens: &mut Tokens) -> Result<Function> {
        let return_type = tokens.consume_type()?;
        let identifier = tokens.consume_identifier()?;
        tokens.consume_parenthesis()?; // consume (
        let argument_types: Vec<ManagedToken> = vec![];
        tokens.consume_parenthesis()?; // consume )

        tokens.consume_bracket()?; // consume {

        let mut block = vec![];
        while let Some(statement) = Statement::parse(tokens)? {
            block.push(statement)
        }
        let return_statement = ReturnStatement::parse(tokens)?;
        tokens.consume_bracket()?; // consume }

        Ok(Function {
            identifier,
            return_type,
            argument_types,
            block,
            return_statement,
        })
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();
        s += &format!("function: {}\n", self.identifier);
        s += &format!("{}return_type: {}\n", get_space(1), self.return_type);
        s += &format!("{}argument_types:\n", get_space(1));
        for argument in self.argument_types.iter() {
            s += &format!("{}{:?}\n", get_space(1), argument);
        }
        s += &format!("{}block:\n", get_space(1));
        for statement in self.block.iter() {
            s += &format!("{}{}", get_space(1), statement.to_string(2));
        }
        s += &format!("{}return_statement:\n", get_space(1));
        s += &self.return_statement.expression_node.to_string(2);
        s
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::tokenize::token::Token;
    use super::super::testutil::*;
    use super::*;

    #[test]
    fn main_func() {
        let actual = Function::parse(&mut Tokens::new(
            vec![
                Token::Type("int".to_owned()),
                Token::Identifier("main".to_owned()),
                Token::Parenthesis("(".to_owned()),
                Token::Parenthesis(")".to_owned()),
                Token::Bracket("{".to_owned()),
                Token::Return,
                Token::Number(10),
                Token::Semicolon,
                Token::Bracket("}".to_owned()),
            ]
            .into_iter()
            .map(|token| ManagedToken::new(token, 0, 0))
            .collect(),
        ))
        .unwrap();

        let expect = Function {
            identifier: mtoken(Token::Identifier("main".to_owned())),
            return_type: mtoken(Token::Type("int".to_owned())),
            argument_types: vec![],
            block: vec![],
            return_statement: ReturnStatement {
                expression_node: *num(10),
            },
        };
        assert_eq!(actual, expect);
    }
}
