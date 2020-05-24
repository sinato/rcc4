use super::super::tokenize::token::{ManagedToken, Token};
use super::super::tokenize::tokens::Tokens;
use super::error::ParseError;
use super::statement::{ReturnStatement, Statement};
use super::util::get_space;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub identifier: ManagedToken,
    pub return_type: ManagedToken,
    pub arguments: Vec<(ManagedToken, ManagedToken)>, // Vec<(identifier, type)>
    pub block: Vec<Statement>,
    pub return_statement: ReturnStatement,
}
impl Function {
    /// function :=
    /// Token::Type Token::Identifier
    ///     Token::Parenthesis("(")
    ///         (Token::Type Token::Identifier (Token::Comma Token::Type Token::Identifier)*)?
    ///     Token::Parenthesis(")")
    ///     Token::Bracket("{")
    ///         statement*
    ///         return_statement
    ///     Token::Bracket("}")
    pub fn parse(tokens: &mut Tokens) -> Result<Function> {
        let return_type = tokens.consume_type()?;
        let identifier = tokens.consume_identifier()?;
        tokens.consume_parenthesis()?; // consume (
        let arguments = Function::consume_arguments(tokens);
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
            arguments,
            block,
            return_statement,
        })
    }

    /// (Token::Type Token::Identifier (Token::Comma Token::Type Token::Identifier)*)?
    fn consume_arguments(tokens: &mut Tokens) -> Vec<(ManagedToken, ManagedToken)> {
        let mut arguments: Vec<(ManagedToken, ManagedToken)> = Vec::new();
        if let Some(token) = tokens.peek() {
            if let Token::Type(_) = token.get_token() {
                let ty = tokens.consume_type().unwrap();
                let identifier = tokens.consume_identifier().unwrap();
                arguments.push((identifier, ty));
                loop {
                    if let Some(token) = tokens.peek() {
                        if let Token::Comma = token.get_token() {
                            tokens.consume_comma().unwrap();
                            let ty = tokens.consume_type().unwrap();
                            let identifier = tokens.consume_identifier().unwrap();
                            arguments.push((identifier, ty));
                            continue;
                        }
                    }
                    break;
                }
            }
        }
        arguments
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();
        s += &format!("function: {}\n", self.identifier);
        s += &format!("{}return_type: {}\n", get_space(1), self.return_type);
        s += &format!("{}arguments:\n", get_space(1));
        for argument in self.arguments.iter() {
            s += &format!("{}{:?}\n", get_space(1 + 1), argument);
        }
        s += &format!("{}block:\n", get_space(1));
        for statement in self.block.iter() {
            s += &format!("{}{}", get_space(1), statement.to_string(2));
        }
        s += &format!("{}return_statement:\n", get_space(1));
        s += &self.return_statement.expression.to_string(2);
        s
    }
}

#[cfg(test)]
mod tests {

    use super::super::super::tokenize::token::Token;
    use super::super::testutil::*;
    use super::*;

    #[cfg(test)]
    mod consume_arguments {

        use super::*;

        #[test]
        fn no_argument() {
            let mut tokens = Tokens::new(vec![mtoken(Token::Parenthesis(")".to_owned()))]);
            let actual = Function::consume_arguments(&mut tokens);
            assert_eq!(actual, vec![]);
        }

        #[test]
        fn one_argument() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Type("int".to_owned())),
                mtoken(Token::Identifier("a".to_owned())),
                mtoken(Token::Parenthesis(")".to_owned())),
            ]);
            let actual = Function::consume_arguments(&mut tokens);
            assert_eq!(
                actual,
                vec![(
                    mtoken(Token::Identifier("a".to_owned())),
                    mtoken(Token::Type("int".to_owned()))
                )]
            );
        }

        #[test]
        fn two_arguments() {
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Type("int".to_owned())),
                mtoken(Token::Identifier("a".to_owned())),
                mtoken(Token::Comma),
                mtoken(Token::Type("int".to_owned())),
                mtoken(Token::Identifier("b".to_owned())),
                mtoken(Token::Parenthesis(")".to_owned())),
            ]);
            let actual = Function::consume_arguments(&mut tokens);
            assert_eq!(
                actual,
                vec![
                    (
                        mtoken(Token::Identifier("a".to_owned())),
                        mtoken(Token::Type("int".to_owned()))
                    ),
                    (
                        mtoken(Token::Identifier("b".to_owned())),
                        mtoken(Token::Type("int".to_owned()))
                    )
                ]
            );
        }
    }
    /*
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
            arguments: vec![],
            block: vec![],
            return_statement: ReturnStatement {
                expression: *num(10),
            },
        };
        assert_eq!(actual, expect);
    }
    */
}
