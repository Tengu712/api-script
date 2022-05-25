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
            panic!("[Lexer error] Try to consume EOF.")
        }
        if cfg!(test) {
            use std::io::Write;
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            let msg = format!("consume '{:?}'\n", self.tokens[self.idx]);
            handle.write_all(msg.as_bytes()).unwrap();
        }
        self.idx += 1;
        self.tokens[self.idx - 1].clone()
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
