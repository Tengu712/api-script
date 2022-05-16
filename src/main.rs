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
    let tmp = String::new();
    for c in code.chars() {
        match c {
            ' ' | '\n' => {
                if c == ' ' {
                    println!("space");
                } else {
                    println!("newline");
                }
            }
            _ => println!("{}", c),
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
        
    }
}
