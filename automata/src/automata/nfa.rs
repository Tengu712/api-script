use super::*;

#[derive(Debug)]
pub struct NFAutomata<T: Clone> {
    pub len: usize,
    pub start: usize,
    pub accepts: HashMap<usize, T>,
    pub transition: Transition,
}
impl<T: Clone> NFAutomata<T> {
    pub fn get_accept_label(&self, state: usize) -> Option<&T> {
        self.accepts.get(&state)
    }
    pub fn connect(&self, other: &Self) -> Self {
        let len = self.len + other.len;
        let start = self.start;
        let mut accepts = self.accepts.clone();
        let mut transition = self.transition.clone();
        if let Some(n) = transition.0.get_mut(&(start, 0)) {
            n.insert(other.start + self.len);
        } else {
            transition
                .0
                .insert((start, 0), HashSet::from([other.start + self.len]));
        }
        for (k, v) in other.accepts.iter() {
            accepts.insert(k + self.len, v.clone());
        }
        for ((s, c), v) in other.transition.0.iter() {
            transition
                .0
                .insert((s + self.len, *c), v.iter().map(|n| n + self.len).collect());
        }
        Self {
            len,
            start,
            accepts,
            transition,
        }
    }
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
#[derive(Debug, Clone)]
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
