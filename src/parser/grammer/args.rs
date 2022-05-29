use super::*;

// <Args> ::= <Arg> (same_indent <Args> | "")
#[derive(Debug)]
pub struct Args {
    pub args: Vec<Arg>,
}
impl Args {
    pub fn parse(tokens: &mut Tokens, indent: usize) -> Self {
        let mut args = Vec::new();
        args.push(Arg::parse(tokens));
        while let Some(Indent(n)) = tokens.look() {
            if *n == indent {
                let _ = tokens.consume();
                args.push(Arg::parse(tokens));
            } else {
                break;
            }
        }
        Self { args }
    }
}
