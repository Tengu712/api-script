mod lexer;
mod parser;
#[cfg(test)]
mod test;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("[fatal error] no file input.");
        return;
    }
    let mut itr = args.iter();
    let _ = itr.next();
    for i in itr {
        let mut tokens = lexer::analyze_tokens_from_file(&i);
        parser::parse(&mut tokens);
    }
}
