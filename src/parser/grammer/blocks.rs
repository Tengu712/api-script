use super::*;

// <Blocks> ::= <Block> (same_indent <Blocks> | "")
#[derive(Debug)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}
impl Blocks {
    pub fn parse(tokens: &mut Tokens, indent: usize) -> Self {
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
