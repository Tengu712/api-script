mod alter;
mod concat;
mod lchar;
mod parser;
mod star;
#[cfg(test)]
mod test;

use alter::*;
use concat::*;
use lchar::*;
use star::*;

use super::nfafragment::*;
use std::collections::VecDeque;

/// A struct to generate a unique state id.
pub struct Context(usize);
impl Context {
    fn new() -> Self {
        Self(0)
    }
    fn next(self) -> (Self, usize) {
        let next = self.0 + 1;
        (Self(next), next)
    }
}

pub trait RegexImpl: std::fmt::Debug {
    fn assemble(&self, context: Context) -> (Context, NFAFrag);
}
pub type Regex = Box<dyn RegexImpl>;

/// A function to parse regex string to regex btree.
pub fn parse_regex(restr: String) -> Regex {
    parser::expr(&mut parser::Token::from_restr(restr))
}
/// A function to parse regex btree to NFA.
pub fn parse_nfa(regex: Regex) -> super::nfa::NFAutomata {
    let (_, frag) = regex.assemble(Context::new());
    frag.build()
}
