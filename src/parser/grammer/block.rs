use super::*;

// <Block> ::=
//   fun <Type> id
//     (deferent_indent args deferent_indent <Args> | "")
//     (deferent_indent logic deferent_indent <Logic> | "")
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
        match tokens.look() {
            Some(Indent(n)) if *n > indent => {
                let _ = tokens.consume();
                match tokens.look() {
                    Some(Args) => {
                        let _ = tokens.consume();
                        let i = tokens.consume_indent();
                        let args = Some(Args::parse(tokens, i));
                        match tokens.look() {
                            Some(Indent(l)) if *l > indent => {
                                let _ = tokens.consume();
                                let _ = tokens.consume_expect(Logic);
                                let i = tokens.consume_indent();
                                Block::FunBlock {
                                    id,
                                    t,
                                    args,
                                    logics: Some(Logics::parse(tokens, i)),
                                }
                            }
                            _ => Block::FunBlock {
                                id,
                                t,
                                args,
                                logics: None,
                            },
                        }
                    }
                    Some(Logic) => {
                        let _ = tokens.consume();
                        let i = tokens.consume_indent();
                        Block::FunBlock {
                            id,
                            t,
                            args: None,
                            logics: Some(Logics::parse(tokens, i)),
                        }
                    }
                    n => error("Block", "'args' or 'logic'", n),
                }
            }
            _ => Block::FunBlock {
                id,
                t,
                args: None,
                logics: None,
            },
        }
    }
}
