use std::collections::{HashMap, HashSet};

pub struct NFAutomata {
    pub start: usize,
    pub accepts: HashSet<usize>,
    pub transition: Transition,
}

/// A struct to make NFA have transition function.
pub struct Transition(pub HashMap<(usize, u8), HashSet<usize>>);
impl Transition {
    pub fn eval(&self, state: usize, chr: u8) -> HashSet<usize> {
        self.0.get(&(state, chr)).unwrap().clone()
    }
}
