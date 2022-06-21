use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct NFAutomata {
    pub start: usize,
    pub accepts: HashSet<usize>,
    pub transition: Transition,
}

/// A struct to make NFA have transition function.
#[derive(Debug)]
pub struct Transition(pub HashMap<(usize, char), HashSet<usize>>);
impl Transition {
    pub fn eval(&self, state: usize, chr: char) -> HashSet<usize> {
        self.0.get(&(state, chr)).unwrap().clone()
    }
}
