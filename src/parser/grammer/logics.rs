use super::*;

// <Logics> ::= <Logic> (same_indent <Logics> | "")
#[derive(Debug)]
pub struct Logics {
    pub logics: Vec<Logic>,
}
impl Logics {
    pub fn parse(tokens: &mut Tokens, indent: usize) -> Self {
        let mut logics = Vec::new();
        logics.push(Logic::parse(tokens, indent));
        while let Some(Indent(n)) = tokens.look() {
            if *n == indent {
                let _ = tokens.consume();
                logics.push(Logic::parse(tokens, indent));
            } else {
                break;
            }
        }
        Self { logics }
    }
}
