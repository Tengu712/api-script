use super::*;

#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub(super) tokens: Vec<Token>,
    pub(super) idx: usize,
}
impl Tokens {
    pub fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, idx: 0 }
    }
    pub fn look(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }
    pub fn consume(&mut self) -> Token {
        if self.idx >= self.tokens.len() {
            panic!("[Lexer error] try to consume EOF.")
        }
        self.idx += 1;
        self.tokens[self.idx - 1].clone()
    }
    pub fn consume_indent(&mut self) -> usize {
        let res = match self.look() {
            Some(Indent(n)) => *n,
            n => panic!("[Lexer error] expected indent, but found '{:?}'.", n),
        };
        let _ = self.consume();
        res
    }
    pub fn consume_expect(&mut self, expect: Token) -> Token {
        if let Some(token) = self.look() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expect) {
                self.consume()
            } else {
                panic!(
                    "[Lexer error] expected '{:?}', but found '{:?}'.",
                    expect, token
                );
            }
        } else {
            panic!("[Lexer error] expected '{:?}', but no token found.", expect);
        }
    }
}
