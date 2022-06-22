use super::*;

#[derive(Debug)]
pub struct NFAutomata {
    pub len: usize,
    pub start: usize,
    pub accepts: HashSet<usize>,
    pub transition: Transition,
}
impl NFAutomata {
    pub fn get_map(&self) -> HashMap<(usize, u8), HashSet<usize>> {
        let mut map = HashMap::new();
        for state in 0..self.len {
            for c in 1..255 {
                if let Some(n) = self.transition.eval(state, c) {
                    map.insert((state, c), self.transition.get_epsilon_sets(n));
                }
            }
        }
        map
    }
}

/// A struct to make NFA have transition function.
#[derive(Debug)]
pub struct Transition(pub HashMap<(usize, u8), HashSet<usize>>);
impl Transition {
    pub fn eval(&self, state: usize, chr: u8) -> Option<&HashSet<usize>> {
        self.0.get(&(state, chr)).clone()
    }
    pub fn get_epsilon_sets(&self, set: &HashSet<usize>) -> HashSet<usize> {
        let mut res = HashSet::new();
        for s in set.iter() {
            res.insert(*s);
            if let Some(n) = self.eval(*s, 0) {
                res = &res | &self.get_epsilon_sets(n);
            }
        }
        res
    }
}
