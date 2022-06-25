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
    /// Trans states until it warps, and return Some(last state).
    /// If it cannot move any state, return None.
    pub fn trans<I>(&self, trg: &mut std::iter::Peekable<I>) -> Option<(usize, String)>
    where
        I: Iterator<Item = u8>,
    {
        let mut buf = String::new();
        let mut flag = false;
        let mut state = self.start;
        while let Some(c) = trg.peek() {
            let tmp = self.transition[state][*c as usize];
            if tmp == 0 {
                break;
            } else {
                buf.push(*c as char);
                flag = true;
                state = tmp;
                let _ = trg.next();
            }
        }
        if flag {
            Some((state, buf))
        } else {
            None
        }
    }
    pub fn start_with<I>(&self, trg: &mut std::iter::Peekable<I>) -> Option<(T, String)>
    where
        I: Iterator<Item = u8>,
    {
        if let Some((state, s)) = self.trans(trg) {
            self.accepts.get(&state).map(|n| n.clone()).map(|n| (n, s))
        } else {
            None
        }
    }
}
