pub mod alter;
pub mod concat;
pub mod lchar;
pub mod star;

pub use alter::*;
pub use concat::*;
pub use lchar::*;
pub use star::*;

use super::nfafragment::*;

pub trait Regex {
    fn assemble(&self, context: Context) -> (Context, NFAFrag);
}

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

pub fn regex_to_nfa(regex: Box<dyn Regex>) -> super::nfa::NFAutomata {
    let (_, frag) = regex.assemble(Context::new());
    frag.build()
}
