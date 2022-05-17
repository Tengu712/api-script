enum Token {
    Fun,
    Args,
    Logic,
    Call,
    Ptr,
    I32,
    U32,
}
fn analyze_tokens(code: &String) -> Vec<Token> {
    let v = Vec::new();
    let mut tmp = String::new();
    let mut indent = 0;
    for c in code.chars() {
        match c {
            _ => tmp.push(c),
        }
    }
    v
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("fatal error: no file input.");
        return;
    }
    for i in args {
        println!("{}", i);
        analyze_tokens(&i);
    }
}
