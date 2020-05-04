```rust
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(u64),
    Type(String)
    Identifier(String),
    Operator(String),
    Bracket(String),
    Parenthesis(String),
    Return,
    Semicolon,
}
```

```
program :=
Token::Type Token::Identifier Token::Parenthesis("(") Token::Parenthesis(")") Token::Bracket("{")
    declare_statement?
    return_statement
Token::Bracket("}")

declare_statement := Token::Type Token::Identifier Token::Semicolon
return_statement := Token::Return expresssion_node Token::Semicolon
expression_node := plus_node
plus_node := mul_node (Token::Operator("+") mul_node)*
mul_node := number_node (Token::Operator("*") number_node)*
number_node := Token::Number
```