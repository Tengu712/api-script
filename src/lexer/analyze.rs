use super::*;

/// A function to split line considering string literal
/// and get number of spaces that's at the top of the line.
fn split_line(trg: String) -> (Vec<String>, usize) {
    let mut v = Vec::new();
    let mut buf = String::new();
    let mut num_spaces = 0;
    let mut is_started = false;
    let mut is_in_word = false;
    let mut is_literal = false;
    for c in trg.chars() {
        if c == '#' {
            break;
        }
        if c == '\"' {
            is_literal = !is_literal;
        }
        if c == ' ' {
            if is_in_word {
                if is_literal {
                    buf.push(c);
                } else {
                    v.push(buf.clone());
                    is_in_word = false;
                }
            } else if !is_started {
                num_spaces += 1;
            }
            continue;
        }
        if !is_in_word {
            is_started = true;
            is_in_word = true;
            buf.clear();
        }
        buf.push(c);
    }
    if buf.len() > 0 {
        v.push(buf.clone());
    }
    (v, num_spaces)
}
pub fn analyze_tokens(lines: Vec<String>) -> Tokens {
    let mut v = Vec::new();
    for line in lines {
        let (words, num_spaces) = split_line(line);
        if words.len() > 0 {
            v.push(Token::Indent(num_spaces));
        }
        for w in words {
            v.push(Token::from(&w));
        }
    }
    v.push(Token::Eof);
    Tokens::from(v)
}
pub fn analyze_tokens_from_file(path: &String) -> Tokens {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    let reader = BufReader::new(File::open(path).unwrap());
    let lines = reader.lines().map(|n| n.unwrap()).collect();
    analyze_tokens(lines)
}
