use super::*;
use super::super::*;

#[test]
fn dfa_check1() {
    let re = regex::parse_regex(String::from("a"));
    let nfa = regex::parse_nfa(re);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(String::from("a")));
    assert_eq!(false, dfa.check(String::from("b")));
}
#[test]
fn dfa_check2() {
    let re = regex::parse_regex(String::from("a|b"));
    let nfa = regex::parse_nfa(re);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(String::from("a")));
    assert_eq!(true, dfa.check(String::from("b")));
    assert_eq!(false, dfa.check(String::from("ba")));
}
#[test]
fn dfa_check3() {
    let re = regex::parse_regex(String::from("(a|bc)*d"));
    let nfa = regex::parse_nfa(re);
    let dfa = dfa::DFAutomata::from(&nfa);
    assert_eq!(true, dfa.check(String::from("d")));
    assert_eq!(true, dfa.check(String::from("aabcad")));
    assert_eq!(true, dfa.check(String::from("bcad")));
    assert_eq!(false, dfa.check(String::from("bc")));
    assert_eq!(false, dfa.check(String::from("abcdd")));
}
