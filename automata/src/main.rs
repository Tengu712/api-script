mod automata;
mod lexer;
mod regex;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    match compile_lexer() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
fn compile_lexer() -> Result<(), String> {
    let reader = BufReader::new(
        File::open(Path::new("./lexer.asl"))
            .map_err(|_| String::from("[fatal error] lexer.asl not found."))?,
    );
    let lines = reader.lines().map(|n| n.unwrap()).collect::<Vec<String>>();
    let mut labels = Vec::new();
    //let mut nfas = Vec::new();
    for (cnt, v) in lines.into_iter().enumerate() {
        if v.len() > 0 && v.chars().nth(0).unwrap() != '#' {
            let splited = v
                .split_whitespace()
                .map(|n| String::from(n))
                .collect::<Vec<String>>();
            if splited.len() < 2 {
                return Err(format!(
                    "[Syntax error] regular expression not found at {} th token.",
                    cnt
                ));
            }
            let restr = process_restr(splited[1].clone());
            labels.push(splited[0].clone());
            println!("{}", restr);
            //nfas.push(regex::parse_nfa(regex::parse_regex(restr), cnt))
        }
    }
    //println!("{:?}", nfas);
    Ok(())
}
fn process_restr(restr: String) -> String {
    let mut res = String::new();
    let mut flag = false;
    for c in restr.chars() {
        match c {
            '_' if flag => {
                res.push('\\');
                res.push('_');
                flag = false;
            }
            '$' if flag => {
                res.push('$');
                flag = false;
            }
            's' if flag => {
                res.push(' ');
                flag = false;
            }
            't' if flag => {
                res.push('\t');
                flag = false;
            }
            'n' if flag => {
                res.push('\n');
                flag = false;
            }
            '$' => flag = true,
            _ => res.push(c),
        }
    }
    res
}
