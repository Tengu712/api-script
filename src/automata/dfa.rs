use super::nfa::*;
use super::*;

pub struct DFAutomata {
    pub start: usize,
    pub accepts: HashSet<usize>,
    pub transition: Vec<[usize; 256]>,
}
impl DFAutomata {
    pub fn from(nfa: &NFAutomata) -> Self {
        let map = nfa.get_map();
        let start_set = nfa.transition.get_epsilon_sets(&HashSet::from([nfa.start]));
        let mut accepts = HashSet::new();
        let mut transition = Vec::new();
        let mut stack = Vec::new();
        let mut states = Vec::new();
        transition.push([0; 256]);
        transition.push([0; 256]);
        stack.push(start_set.clone());
        states.push(start_set);
        while let Some(set) = stack.pop() {
            let trans_id_now = states.iter().position(|v| v == &set).unwrap() + 1;
            if !set.is_disjoint(&nfa.accepts) {
                accepts.insert(trans_id_now);
            }
            for c in 1..255 {
                let mut transed = HashSet::new();
                for state in set.iter() {
                    if let Some(n) = map.get(&(*state, c)) {
                        transed = &transed | n;
                    }
                }
                if !transed.is_empty() {
                    if let Some(n) = states.iter().position(|v| v == &transed) {
                        transition[trans_id_now][c as usize] = n + 1;
                    } else {
                        transition.push([0; 256]);
                        stack.push(transed.clone());
                        states.push(transed.clone());
                        transition[trans_id_now][c as usize] = states.len();
                    }
                }
            }
        }
        Self {
            start: 1,
            accepts,
            transition,
        }
    }
    pub fn check(&self, trg: String) -> bool {
        let mut state = self.start;
        for c in trg.as_bytes() {
            state = self.transition[state][*c as usize];
        }
        self.accepts.contains(&state)
    }
}
