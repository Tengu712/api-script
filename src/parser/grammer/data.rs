use super::*;

// <Data> ::= nullptr | str | int | float | id
#[derive(Debug)]
pub struct Data {
    pub value: Token,
}
impl Data {
    pub fn parse(tokens: &mut Tokens) -> Self {
        let value = match tokens.look() {
            Some(Nullptr) | Some(Str(_)) | Some(Int(_)) | Some(Float(_)) | Some(Id(_)) => {
                tokens.consume()
            }
            n => error("Data", "data token", n),
        };
        Self { value }
    }
}
