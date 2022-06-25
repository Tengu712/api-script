use super::super::*;
use super::*;

impl<T: Clone> dfa::DFAutomata<T> {
    fn check(&self, s: &str) -> bool {
        let mut state = self.start;
        for c in s.as_bytes() {
            state = self.transition[state][*c as usize];
        }
        self.accepts.get(&state).is_some()
    }
}

#[test]
fn dfa_check1() {
    let re = regex::parse_regex(String::from("a"));
    let nfa = regex::parse_nfa(re, 0);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(&String::from("a")));
    assert_eq!(false, dfa.check(&String::from("b")));
}
#[test]
fn dfa_check2() {
    let re = regex::parse_regex(String::from("a|b"));
    let nfa = regex::parse_nfa(re, 0);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(&String::from("a")));
    assert_eq!(true, dfa.check(&String::from("b")));
    assert_eq!(false, dfa.check(&String::from("ba")));
}
#[test]
fn dfa_check3() {
    let re = regex::parse_regex(String::from("(a|bc)*d"));
    let nfa = regex::parse_nfa(re, 0);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(&String::from("d")));
    assert_eq!(true, dfa.check(&String::from("aabcad")));
    assert_eq!(true, dfa.check(&String::from("bcad")));
    assert_eq!(false, dfa.check(&String::from("bc")));
    assert_eq!(false, dfa.check(&String::from("abcdd")));
}
#[test]
fn dfa_check4() {
    let re = regex::parse_regex(String::from("a|b|$"));
    let nfa = regex::parse_nfa(re, 0);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(&String::from("")));
    assert_eq!(true, dfa.check(&String::from("a")));
    assert_eq!(true, dfa.check(&String::from("b")));
    assert_eq!(false, dfa.check(&String::from("ba")));
}
#[test]
fn dfa_start_with() {
    let nfa1 = regex::parse_nfa(regex::parse_regex(String::from("a|b|$")), 0);
    let nfa2 = regex::parse_nfa(regex::parse_regex(String::from("(a|bc)*d")), 1);
    let dfa = dfa::DFAutomata::from(&nfa1.connect(&nfa2));
    let cl = |m, l, s: &str| {
        assert_eq!(
            if m { Some((l, String::from(s))) } else { None },
            dfa.start_with(&mut s.as_bytes().iter().copied().peekable())
        );
    };
    cl(true, 0, "b");
    cl(true, 1, "d");
    cl(true, 1, "abcad");
    cl(false, 0, "");
    cl(false, 0, "fbc");
}
