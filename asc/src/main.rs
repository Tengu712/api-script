mod lexer;
mod preprocessor;

use aslp::lexer::RuntimeLexer;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("[Fatal error] no file input.");
        return;
    }
    let rlex = RuntimeLexer::new();
    let itr = args.iter().skip(1);
    for path in itr {
        match process(path, &rlex) {
            Ok(()) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}
fn process(path: &String, rlex: &RuntimeLexer) -> Result<(), String> {
    let reader = BufReader::new(File::open(Path::new(path)).unwrap());
    let lines = reader.lines().map(|n| n.unwrap()).collect();
    let src = preprocessor::preprocess(lines);
    let tokens = lexer::compile(rlex, src);
    println!("{:?}", tokens);
    Ok(())
}
