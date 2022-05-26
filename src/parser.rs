pub mod grammer;
#[cfg(test)]
pub mod test;

use grammer::*;

use super::lexer::*;

pub fn parse(tokens: &mut Tokens) -> Program {
    Program::parse(tokens)
}
