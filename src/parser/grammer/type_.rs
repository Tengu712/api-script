use super::*;

// <Type> ::= void | ptr | i32 | u32
#[derive(Debug)]
pub struct Type {
    pub value: Token,
}
impl Type {
    pub fn parse(tokens: &mut Tokens) -> Self {
        let value = match tokens.look() {
            Some(Void) | Some(Ptr) | Some(I32) | Some(U32) => tokens.consume(),
            n => error("Type", "type token", n),
        };
        Self { value }
    }
}
