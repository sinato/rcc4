```rust
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Number(u64),
    Type(String),
    Identifier(String),
    Operator(String),
    Bracket(String),
    SBracket(String),
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
declare_statement := type token::semicolon
expression_statement := expression token::semicolon
return_statement := Token::Return expresssion_node Token::Semicolon

type := Token::Type Token::Identifier (Token::SBracket expression Token::SBracket)*

expression := exp2
exp2  := exp12 | exp12 (Token::Operator("=") exp12)+
exp12 := exp13 | exp13 (Token::Operator("+") exp13)+
exp13 := exp16 | exp16 (Token::Operator("*") exp16)*
exp16 := Token::Number | Token::Identifier | function_call
function_call := Token::Identifier Token::Parenthesis("(") (expression (Token::Comma expresssion_node)*)? Token::Parenthesis(")")
```