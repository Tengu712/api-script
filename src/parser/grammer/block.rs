use super::*;

// <Block> ::=
//   fun <Type> id (deferent_indent1
//     (  args deferent_indent2 <Args>
//           (deferent_indent1 logic deferent_indent3 <Logic> | "")
//      | logic deferent_indent3 <Logic>)
#[derive(Debug)]
pub enum Block {
    FunBlock {
        t: Type,
        id: Token,
        args: Option<Args>,
        logics: Option<Logics>,
    },
}
impl Block {
    pub fn parse(tokens: &mut Tokens, indent: usize) -> Self {
        match tokens.look() {
            Some(Fun) => {
                let _ = tokens.consume();
                Block::parse_function(tokens, indent)
            }
            n => error("Block", "'fun'", n),
        }
    }
    fn parse_function(tokens: &mut Tokens, indent: usize) -> Self {
        let t = Type::parse(tokens);
        let id = tokens.consume_expect(Id(String::new()));
        let (args, logics) = match tokens.look() {
            Some(Indent(n)) if *n > indent => {
                let deferent_indent1 = tokens.consume_indent();
                match tokens.look() {
                    Some(Args) => found_args(tokens, deferent_indent1),
                    Some(Logic) => found_logic(tokens),
                    n => error("Block", "'args' or 'logic'", n),
                }
            }
            _ => (None, None),
        };
        Block::FunBlock {
            id,
            t,
            args,
            logics,
        }
    }
}
fn found_args(tokens: &mut Tokens, deferent_indent1: usize) -> (Option<Args>, Option<Logics>) {
    let _ = tokens.consume();
    let deferent_indent2 = tokens.consume_indent();
    let args = Args::parse(tokens, deferent_indent2);
    match tokens.look() {
        Some(Indent(n)) if *n == deferent_indent1 => {
            let _ = tokens.consume();
            let _ = tokens.consume_expect(Logic);
            let deferent_indent3 = tokens.consume_indent();
            (Some(args), Some(Logics::parse(tokens, deferent_indent3)))
        }
        _ => (Some(args), None),
    }
}
fn found_logic(tokens: &mut Tokens) -> (Option<Args>, Option<Logics>) {
    let _ = tokens.consume();
    let deferent_indent3 = tokens.consume_indent();
    (None, Some(Logics::parse(tokens, deferent_indent3)))
}
