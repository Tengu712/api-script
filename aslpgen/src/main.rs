mod automata;
mod regex;
mod lexer;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

fn main() {
    match lexer::compile_lexer() {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e),
    }
}
