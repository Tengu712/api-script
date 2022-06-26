use super::nfa::*;
use super::*;

#[derive(Debug)]
pub struct DFAutomata<T: Clone> {
    pub start: usize,
    pub accepts: HashMap<usize, T>,
    pub transition: Vec<[usize; 256]>,
}
impl<T: Clone> DFAutomata<T> {
    pub fn from(nfa: &NFAutomata<T>) -> Self {
        let map = nfa.get_map();
        let start_set = nfa.transition.get_epsilon_sets(&HashSet::from([nfa.start]));
        let mut accepts = HashMap::new();
        let mut transition = Vec::new();
        let mut stack = Vec::new();
        let mut states = Vec::new();
        transition.push([0; 256]);
        transition.push([0; 256]);
        stack.push(start_set.clone());
        states.push(start_set);
        while let Some(set) = stack.pop() {
            let trans_id_now = states.iter().position(|v| v == &set).unwrap() + 1;
            for i in set.iter() {
                if let Some(n) = nfa.get_accept_label(*i) {
                    accepts.insert(trans_id_now, n.clone());
                }
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
}
