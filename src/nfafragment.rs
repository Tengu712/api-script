use super::nfa::*;
use std::collections::{HashMap, HashSet};

/// A struct to build a nfa.
pub struct NFAFrag {
    pub start: usize,
    pub accepts: HashSet<usize>,
    pub map: HashMap<(usize, char), HashSet<usize>>,
}
impl NFAFrag {
    pub fn new(start: usize) -> Self {
        Self {
            start,
            accepts: HashSet::new(),
            map: HashMap::new(),
        }
    }
    pub fn compose(op1: &NFAFrag, op2: &NFAFrag, start: usize) -> NFAFrag {
        let mut new = NFAFrag::new(start);
        new.map = op1.map.clone();
        for (k, map_accepts2) in op2.map.iter() {
            if let Some(map_accepts1) = op1.map.get(k) {
                new.map.insert(*k, map_accepts1 | map_accepts2);
            } else {
                new.map.insert(*k, map_accepts2.clone());
            }
        }
        new
    }
    pub fn connect(&mut self, from: usize, chr: char, to: usize) {
        if let Some(map_accepts) = self.map.get_mut(&(from, chr)) {
            map_accepts.insert(to);
        } else {
            let mut map_accepts = HashSet::new();
            map_accepts.insert(to);
            self.map.insert((from, chr), map_accepts);
        }
    }
    pub fn build(self) -> NFAutomata {
        NFAutomata {
            start: self.start,
            accepts: self.accepts,
            transition: Transition(self.map),
        }
    }
}
