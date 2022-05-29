use super::*;

// <Arg> ::= <Type> <Data>
#[derive(Debug)]
pub struct Arg {
    pub t: Type,
    pub d: Data,
}
impl Arg {
    pub fn parse(tokens: &mut Tokens) -> Self {
        let t = Type::parse(tokens);
        let d = Data::parse(tokens);
        println!("{:?}", d);
        Self { t, d }
    }
}
