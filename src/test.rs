use super::lexer::*;
use std::io::Write;

/*
impl Cell {
    fn print(&self, handle: &mut std::io::StdoutLock, indent: usize) {
        use std::io::Write;
        let mut indent_str = String::new();
        for _ in 0..indent {
            indent_str.push(' ');
            indent_str.push(' ');
        }
        match self {
            Cell::Nil => handle.write_all((indent_str + "Nil ").as_bytes()).unwrap(),
            Cell::Atom(n) => {
                let f = format!("{}{:?} ", indent_str, n);
                handle.write_all(f.as_bytes()).unwrap();
            }
            Cell::Pair(car, cdr) => {
                car.print(handle, indent);
                cdr.print(handle, indent);
            }
        }
    }
}*/

#[cfg(test)]
const CODE1: &'static str = "
# Hello World with MessageBoxA
fun big_func
  logic
    call 4args_func
      i32 0
      i32 0
      i32 0
      i32 0
    call 2args_func
      u32 0
      u32 0
";
#[test]
fn lexer_test1() {
    let tokens = analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let expect = "
Indent(0) Fun Id(\"big_func\")
Indent(2) Logic
Indent(4) Call Id(\"4args_func\")
Indent(6) I32 Int(\"0\")
Indent(6) I32 Int(\"0\")
Indent(6) I32 Int(\"0\")
Indent(6) I32 Int(\"0\")
Indent(4) Call Id(\"2args_func\")
Indent(6) U32 Int(\"0\")
Indent(6) U32 Int(\"0\")
Indent(0) Eof
";
    assert_eq!(tokens.format(), expect);
}
#[test]
fn parser_test1() {
    let mut tokens = analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    // parse(&mut tokens).print(&mut handle, 0);
}
