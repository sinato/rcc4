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
    Comma,
    Semicolon,
}
```

```
program := function+

function :=
Token::Type Token::Identifier
    Token::Parenthesis("(")
        (Token::Type Token::Identifier (Token::Comma Token::Type Token::Identifier)*)?
    Token::Parenthesis(")")
    Token::Bracket("{")
        statement*
        return_statement
    Token::Bracket("}")

statement := declare_statement | expression_statement
declare_statement := token::type token::identifier token::semicolon
expression_statement := expression_node token::semicolon
return_statement := Token::Return expresssion_node Token::Semicolon
expression_node := eq_node
eq_node := plus_node (Token::Operator("=") plus_node)*
plus_node := mul_node (Token::Operator("+") mul_node)*
mul_node := fn_call_node (Token::Operator("*") fn_call_node)*
fn_call_node := Token::Identifier Token::Parenthesis("(") (expression_node (Token::Comma expresssion_node)*)? Token::Parenthesis(")") | leaf_node
leaf_node := Token::Number | Token::Identifier
```