use super::*;

// <Logic> ::= call <Type> id (deferent_indent <Args> | "")
#[derive(Debug)]
pub enum Logic {
    CallLogic {
        t: Type,
        id: Token,
        args: Option<Args>,
    },
}
impl Logic {
    pub fn parse(tokens: &mut Tokens, indent: usize) -> Self {
        match tokens.look() {
            Some(Call) => {
                let _ = tokens.consume();
                Logic::parse_call(tokens, indent)
            }
            n => error("Logic", "'call'", n),
        }
    }
    fn parse_call(tokens: &mut Tokens, indent: usize) -> Self {
        let t = Type::parse(tokens);
        let id = tokens.consume_expect(Id(String::new()));
        let new_indent = match tokens.look() {
            Some(Indent(n)) if *n > indent => tokens.consume_indent(),
            _ => return Logic::CallLogic { t, id, args: None },
        };
        Logic::CallLogic {
            t,
            id,
            args: Some(Args::parse(tokens, new_indent)),
        }
    }
}
