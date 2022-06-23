mod automata;
mod lexer;
mod preprocessor;
mod regex;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("[fatal error] no file input.");
        return;
    }
    let itr = args.iter().skip(1);
    for src_path in itr {
        match compile(src_path) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}
fn compile(src_path: &String) -> Result<(), String> {
    let reader = BufReader::new(File::open(Path::new(src_path)).unwrap());
    let lines = reader.lines().map(|n| n.unwrap()).collect();
    let src = preprocessor::preprocess(lines)?;
    let tokens = lexer::parse_tokens(src);
    println!("{:?}", tokens);
    Ok(())
}
