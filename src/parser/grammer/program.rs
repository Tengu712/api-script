use super::*;

// <Program> ::= indent <Blocks> $
#[derive(Debug)]
pub struct Program {
    pub blocks: Blocks,
}
impl Program {
    pub fn parse(tokens: &mut Tokens) -> Self {
        let indent = tokens.consume_indent();
        let blocks = Blocks::parse(tokens, indent);
        let _ = tokens.consume_expect(Eof);
        Self { blocks }
    }
}
