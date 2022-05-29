use super::*;

fn error(place: &str, expect: &str, found: Option<&Token>) -> ! {
    if let Some(n) = found {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but '{:?}' found.",
                place, expect, n
            )
        );
    } else {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but no token found.",
                place, expect
            )
        );
    }
}

pub fn parse(tokens: &mut Tokens) -> Program {
    Program::parse(tokens)
}

// <Program> ::= indent <Blocks> $
#[derive(Debug)]
pub struct Program {
    pub blocks: Blocks,
}
impl Program {
    fn parse(tokens: &mut Tokens) -> Self {
        let indent = tokens.consume_indent();
        let blocks = Blocks::parse(tokens, indent);
        let _ = tokens.consume_expect(Eof);
        Self { blocks }
    }
}
// <Blocks> ::= <Block> (same_indent <Blocks> | "")
#[derive(Debug)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}
impl Blocks {
    fn parse(tokens: &mut Tokens, indent: usize) -> Self {
        let mut blocks = Vec::new();
        blocks.push(Block::parse(tokens, indent));
        while let Some(Indent(n)) = tokens.look() {
            if *n == indent {
                let _ = tokens.consume();
                blocks.push(Block::parse(tokens, indent));
            } else {
                break;
            }
        }
        Blocks { blocks }
    }
}
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
    fn parse(tokens: &mut Tokens, indent: usize) -> Self {
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
// <Logics> ::= <Logic> (same_indent <Logics> | "")
#[derive(Debug)]
pub struct Logics {
    pub logics: Vec<Logic>,
}
impl Logics {
    fn parse(tokens: &mut Tokens, indent: usize) -> Self {
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
    fn parse(tokens: &mut Tokens, indent: usize) -> Self {
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
// <Args> ::= <Arg> (same_indent <Args> | "")
#[derive(Debug)]
pub struct Args {
    pub args: Vec<Arg>,
}
impl Args {
    fn parse(tokens: &mut Tokens, indent: usize) -> Self {
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
// <Arg> ::= <Type> <Data>
#[derive(Debug)]
pub struct Arg {
    pub t: Type,
    pub d: Data,
}
impl Arg {
    fn parse(tokens: &mut Tokens) -> Self {
        let t = Type::parse(tokens);
        let d = Data::parse(tokens);
        println!("{:?}", d);
        Self { t, d }
    }
}
// <Type> ::= void | ptr | i32 | u32
#[derive(Debug)]
pub struct Type {
    pub value: Token,
}
impl Type {
    fn parse(tokens: &mut Tokens) -> Self {
        let value = match tokens.look() {
            Some(Void) | Some(Ptr) | Some(I32) | Some(U32) => tokens.consume(),
            n => error("Type", "type token", n),
        };
        Self { value }
    }
}
// <Data> ::= nullptr | str | int | float | id
#[derive(Debug)]
pub struct Data {
    pub value: Token,
}
impl Data {
    fn parse(tokens: &mut Tokens) -> Self {
        let value = match tokens.look() {
            Some(Nullptr) | Some(Str(_)) | Some(Int(_)) | Some(Float(_)) | Some(Id(_)) => {
                tokens.consume()
            }
            n => error("Data", "data token", n),
        };
        Self { value }
    }
}
