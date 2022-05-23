use super::{
    lexer::{Token::*, *},
    parser::*,
};

#[cfg(test)]
const CODE1: &'static str = "
# Hello World with MessageBoxA
fun hello_world
  logic
    call user32.MessageBoxA
      ptr nullptr
      ptr \"Hello World\"
      ptr \"Title\"
      i32 0
";

#[test]
fn lexer_test1() {
    let expect = [
        Fun,
        Id(String::from("hello_world")),
        Indent,
        Logic,
        Indent,
        Call,
        Id(String::from("user32.MessageBoxA")),
        Indent,
        Ptr,
        Nullptr,
        Ptr,
        Str(String::from("\"Hello World\"")),
        Ptr,
        Str(String::from("\"Title\"")),
        I32,
        Int(String::from("0")),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect()),
        Tokens::from(expect),
    );
}

#[test]
fn parser_test1() {
    let mut tokens = analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    parse(&mut tokens);
}
