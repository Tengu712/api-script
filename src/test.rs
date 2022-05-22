use super::lexer::{Token::*, *};

#[test]
fn lexer_test1() {
    let code: &'static str = "
# Hello World with MessageBoxA
fun hello_world
  args
  logic
    call user32.MessageBoxA
      ptr nullptr
      ptr \"Hello World\"
      ptr \"Title\"
      i32 0
";
    let expect = [
        Fun,
        Id(String::from("hello_world")),
        Indent,
        Args,
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
    ];
    assert_eq!(
        analyze_tokens(code.split('\n').map(|n| String::from(n)).collect()),
        expect
    );
}
