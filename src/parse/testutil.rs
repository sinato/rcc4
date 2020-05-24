use super::super::tokenize::token::{ManagedToken, Token};

pub fn mtoken(token: Token) -> ManagedToken {
    ManagedToken::new(token, 0, 0)
}
