mod automata;
mod regex;

fn main() {
    let re = regex::parse_regex(String::from("a|b"));
    let nfa = regex::parse_nfa(re);
    let dfa = automata::dfa::DFAutomata::from(&nfa);

    let s = format!("{:?}", dfa.transition);
    let s = s.replace("[", "");
    let s = s.replace("], ", "]");
    let mut b = String::new();
    for i in s.chars() {
        if i == ']' {
            b.push('\n');
        } else {
            b.push(i);
        }
    }
    println!("{}", b);
}
