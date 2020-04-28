use super::super::tokenize::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub operator: Token,
    pub operand: Vec<Box<Node>>,
}

impl Node {
    pub fn get_operator_clone(&self) -> Token {
        self.operator.clone()
    }
    pub fn get_operand(self) -> Vec<Box<Node>> {
        self.operand
    }
    pub fn create_single_node(token: Token) -> Box<Node> {
        Box::new(Node {
            operator: token,
            operand: Vec::new(),
        })
    }
    pub fn to_string(&self, tab_level: u32) -> String {
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
