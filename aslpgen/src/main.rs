mod automata;
mod regex;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
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
    let mut nfas = Vec::new();
    let mut cnt = 0;
    for v in lines.into_iter() {
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
            nfas.push(regex::parse_nfa(regex::parse_regex(restr), cnt));
            cnt += 1;
        }
    }
    let mut itr = nfas.into_iter();
    let mut nfa = itr.next().unwrap();
    for i in itr {
        nfa = nfa.connect(&i);
    }
    let dfa = automata::dfa::DFAutomata::from(&nfa);
    let mut writer = BufWriter::new(
        File::create(Path::new("./lexer_automata.rs"))
            .map_err(|_| String::from("[fatal error] lexer.rs not created."))?,
    );
    write_lexer(
        &mut writer,
        String::from("#[derive(Debug, Clone)]\npub enum Token {\n"),
    )?;
    for i in labels.iter() {
        write_lexer(&mut writer, format!("    {},\n", i.clone()))?;
    }
    write_lexer(
        &mut writer,
        format!(
            "}}\npub const ACCEPTS: [(usize, Token); {}] = [\n",
            dfa.accepts.len()
        ),
    )?;
    for (k, v) in dfa.accepts.into_iter() {
        let label_splited = labels[v].split('(').collect::<Vec<&str>>();
        let f = if label_splited.len() < 2 {
            format!("    ({}, Token::{}),\n", k, labels[v])
        } else if label_splited[1] == "usize)" {
            format!("    ({}, Token::{}(0)),\n", k, label_splited[0])
        } else if label_splited[1] == "String)" {
            format!("    ({}, Token::{}(String::new())),\n", k, label_splited[0])
        } else {
            panic!("[Syntax error] invalid attribute : {}", label_splited[1]);
        };
        write_lexer(&mut writer, f)?;
    }
    write_lexer(
        &mut writer,
        format!(
            "];\npub const TRANSITION: [[usize; 256]; {}] = [\n",
            dfa.transition.len()
        ),
    )?;
    for i in dfa.transition {
        write_lexer(&mut writer, format!("    {:?},\n", i))?;
    }
    write_lexer(&mut writer, String::from("];\n"))?;
    Ok(())
}
fn process_restr(restr: String) -> String {
    let mut res = String::new();
    let mut chars = restr.chars();
    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let start = chars.next().unwrap() as u8;
                if chars.next().unwrap() != '-' {
                    panic!("[Syntax error] '-' expected.");
                }
                let end = chars.next().unwrap() as u8;
                if chars.next().unwrap() != ']' {
                    panic!("[Syntax error] ']' expected.");
                }
                res.push('(');
                for i in start..end {
                    res.push(i as char);
                    res.push('|');
                }
                res.push(end as char);
                res.push(')');
            }
            '$' => match chars.next().unwrap() {
                '_' => {
                    res.push('\\');
                    res.push('_');
                }
                '$' => res.push('$'),
                's' => res.push(' '),
                't' => res.push('\t'),
                'n' => res.push('\n'),
                n => panic!("[Syntax error] invalid escape sequence. : {}", n),
            },
            _ => res.push(c),
        }
    }
    res
}
fn write_lexer(writer: &mut BufWriter<File>, f: String) -> Result<(), String> {
    writer
        .write_all(f.as_bytes())
        .map_err(|_| String::from("[IO error] something is wrong with output lexer.rs."))
}
