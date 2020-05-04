use super::super::tokenize::token::{ManagedToken, Token};

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionNode {
    pub identifier: ManagedToken,
    pub return_type: ManagedToken,
    pub argument_types: Vec<ManagedToken>,
    pub block: Vec<ExpressionNode>,
}
impl FunctionNode {
    pub fn new(
        identifier: ManagedToken,
        return_type: ManagedToken,
        argument_types: Vec<ManagedToken>,
        block: Vec<ExpressionNode>,
    ) -> FunctionNode {
        FunctionNode {
            identifier,
            return_type,
            argument_types,
            block,
        }
    }
    pub fn to_string(&self) -> String {
        let mut s = "".to_owned();
        s += "AST ============================\n";
        s += &format!("function: {}\n", self.identifier);
        s += &format!("return_type: {}\n", self.return_type);
        s += &format!("argument_types:\n");
        for argument in self.argument_types.iter() {
            s += &format!("{:?}\n", argument);
        }
        s += &format!("block:\n");
        for expression in self.block.iter() {
            s += &expression.to_string(1);
        }
        s += "================================\n";
        s
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionNode {
    pub operator: Token,
    pub operand: Vec<Box<ExpressionNode>>,
}

impl ExpressionNode {
    pub fn new(operator: Token, operand: Vec<Box<ExpressionNode>>) -> ExpressionNode {
        ExpressionNode { operator, operand }
    }
    pub fn get_operator_clone(&self) -> Token {
        self.operator.clone()
    }
    pub fn get_operand(self) -> Vec<Box<ExpressionNode>> {
        self.operand
    }
    pub fn create_single_node(token: Token) -> Box<ExpressionNode> {
        Box::new(ExpressionNode {
            operator: token,
            operand: Vec::new(),
        })
    }
    fn to_string(&self, tab_level: u32) -> String {
        let mut s = "".to_owned();
        s += &format!("{}{}\n", get_space(tab_level), self.operator);
        for val in self.operand.iter() {
            s += &format!("{}{}", get_space(tab_level), val.to_string(tab_level + 1));
        }
        s
    }
}

fn get_space(tab_level: u32) -> String {
    (0..tab_level)
        .map(|_| "| ".to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn get_space_tab_level_two() {
        assert_eq!(get_space(2), "| | ".to_string());
    }
}
