use super::error::ParseError;
use super::util::get_space;
use rcc_syntax::token::Token;
use rcc_syntax::tokens::Tokens;
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
pub struct Expression {
    pub expression: Exp2,
}
impl Expression {
    pub fn parse(tokens: &mut Tokens) -> Result<Expression> {
        Ok(Expression {
            expression: Exp2::parse(tokens)?,
        })
    }
    pub fn to_string(&self, space_num: u32) -> String {
        format!("{}", self.expression.to_string(space_num))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp2 {
    Single(Exp12),
    Eq(Vec<Exp12>),
}
impl Exp2 {
    fn parse(tokens: &mut Tokens) -> Result<Exp2> {
        let mut operand = vec![Exp12::parse(tokens)?];
        while let Some(_token) = tokens.check_next_operator("=") {
            tokens.next(); // consume "="
            operand.push(Exp12::parse(tokens)?);
        }
        if operand.len() == 1 {
            Ok(Exp2::Single(operand.remove(0)))
        } else {
            Ok(Exp2::Eq(operand))
        }
    }
    pub fn to_string(&self, space_num: u32) -> String {
        match self {
            Exp2::Single(exp) => format!("{}", exp.to_string(space_num)),
            Exp2::Eq(exps) => {
                let mut s = format!("{}operator: =\n", get_space(space_num));
                for exp in exps {
                    s += &format!("{}\n", exp.to_string(space_num + 1))
                }
                s
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp12 {
    Single(Exp13),
    Add(Vec<Exp13>),
}
impl Exp12 {
    fn parse(tokens: &mut Tokens) -> Result<Exp12> {
        let mut operand = vec![Exp13::parse(tokens)?];
        while let Some(_token) = tokens.check_next_operator("+") {
            tokens.next(); // consume "+"
            operand.push(Exp13::parse(tokens)?);
        }
        if operand.len() == 1 {
            Ok(Exp12::Single(operand.remove(0)))
        } else {
            Ok(Exp12::Add(operand))
        }
    }
    pub fn to_string(&self, space_num: u32) -> String {
        match self {
            Exp12::Single(exp) => format!("{}", exp.to_string(space_num)),
            Exp12::Add(exps) => {
                let mut s = format!("{}operator: +\n", get_space(space_num));
                for exp in exps {
                    s += &format!("{}\n", exp.to_string(space_num + 1))
                }
                s
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp13 {
    Single(Exp16),
    Mul(Vec<Exp16>),
}
impl Exp13 {
    fn parse(tokens: &mut Tokens) -> Result<Exp13> {
        let mut operand = vec![Exp16::parse(tokens)?];
        while let Some(_token) = tokens.check_next_operator("*") {
            tokens.next(); // consume "*"
            operand.push(Exp16::parse(tokens)?);
        }
        if operand.len() == 1 {
            Ok(Exp13::Single(operand.remove(0)))
        } else {
            Ok(Exp13::Mul(operand))
        }
    }
    pub fn to_string(&self, space_num: u32) -> String {
        match self {
            Exp13::Single(exp) => format!("{}", exp.to_string(space_num)),
            Exp13::Mul(exps) => {
                let mut s = format!("{}operator: *\n", get_space(space_num));
                for exp in exps {
                    s += &format!("{}\n", exp.to_string(space_num + 1))
                }
                s
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Exp16 {
    Number(u64),
    Identifier(String),
    FunctionCall(String, Vec<Expression>),
}
impl Exp16 {
    fn parse(tokens: &mut Tokens) -> Result<Exp16> {
        if let Some(token) = tokens.peek() {
            if let Token::Number(_) = token.get_token() {
                let num = tokens.next().unwrap().get_token().get_number().unwrap();
                return Ok(Exp16::Number(num));
            }
            if let Token::Identifier(_) = token.get_token() {
                if let Some(token2) = tokens.peek2() {
                    if let Token::Parenthesis(_) = token2.get_token() {
                        let identifier =
                            tokens.next().unwrap().get_token().get_identifier().unwrap();
                        tokens.next(); // consume (

                        // match (expression_node (Token::Comma expresssion_node)*)?
                        let mut parameters: Vec<Expression> = vec![];
                        if tokens.check_next_is_expression_node() {
                            parameters.push(Expression::parse(tokens)?);
                        }
                        loop {
                            if let Some(token) = tokens.peek() {
                                if let Token::Comma = token.get_token() {
                                    tokens.next(); // consume ,
                                    parameters.push(Expression::parse(tokens)?);
                                    continue;
                                }
                            }
                            break;
                        }
                        tokens.next(); // consume )
                        return Ok(Exp16::FunctionCall(identifier, parameters));
                    }
                }
                let identifier = tokens.next().unwrap().get_token().get_identifier().unwrap();
                return Ok(Exp16::Identifier(identifier));
            }
        }
        Err(ParseError::Unexpect(tokens.next()))
    }
    pub fn to_string(&self, space_num: u32) -> String {
        match self {
            Exp16::Number(num) => format!("{}{}", get_space(space_num), num),
            Exp16::Identifier(identifier) => format!("{}{}", get_space(space_num), identifier),
            Exp16::FunctionCall(identifier, exps) => {
                let mut s = format!("{}function_call: {}\n", get_space(space_num), identifier);
                for exp in exps {
                    s += &format!("{}\n", exp.to_string(space_num + 1))
                }
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::testutil::*;
    use super::*;

    #[cfg(test)]
    mod tests_parse_expression {

        use super::*;

        #[test]
        fn parse_expression() {
            // a = 10 + 20 * func(30)
            let mut tokens = Tokens::new(vec![
                mtoken(Token::Identifier("a".to_owned())),
                mtoken(Token::Operator("=".to_owned())),
                mtoken(Token::Number(10)),
                mtoken(Token::Operator("+".to_owned())),
                mtoken(Token::Number(20)),
                mtoken(Token::Operator("*".to_owned())),
                mtoken(Token::Identifier("func".to_owned())),
                mtoken(Token::Parenthesis("(".to_owned())),
                mtoken(Token::Number(30)),
                mtoken(Token::Parenthesis(")".to_owned())),
            ]);

            let actual = Expression::parse(&mut tokens).unwrap();

            let num_10 = Exp13::Single(Exp16::Number(10));
            let num_20 = Exp16::Number(20);
            let parameter = Expression {
                expression: Exp2::Single(Exp12::Single(Exp13::Single(Exp16::Number(30)))),
            };
            let func_call = Exp16::FunctionCall("func".to_owned(), vec![parameter]);
            let mul = Exp13::Mul(vec![num_20, func_call]);
            let add = Exp12::Add(vec![num_10, mul]);
            let ide = Exp12::Single(Exp13::Single(Exp16::Identifier("a".to_owned())));
            let eq = Exp2::Eq(vec![ide, add]);
            let expect = Expression { expression: eq };
            assert_eq!(actual, expect);
        }
    }
}
