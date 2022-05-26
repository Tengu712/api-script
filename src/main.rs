mod formatter;
mod lexer;
mod parser;
#[cfg(test)]
mod test;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("[fatal error] no file input.");
        return;
    } else if args.len() < 4 {
        eprintln!("[fatal error] command line arguments is fewer than expected.");
        return;
    }
    let file_path = args.get(1).unwrap();
    let target_language = args.get(2).unwrap();
    let mut tokens = lexer::analyze_tokens_from_file(file_path);
    let ast = parser::parse(&mut tokens);
    let res = match target_language.as_str() {
        "rust" => ast.format_for_rust(),
        n => {
            eprintln!("[fatal error] {} is not supported now.", n);
            return;
        }
    };
    match args.get(3) {
        Some(n) if n == "print" => println!("{}", res),
        Some(n) => {
            use std::io::Write;
            let mut file = std::fs::File::create(n).unwrap();
            file.write_all(res.as_bytes()).unwrap();
            file.flush().unwrap();
        },
        _ => (),
    }
}
