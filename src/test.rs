use super::*;

#[cfg(test)]
const CODE1: &'static str = "
# Hello World with MessageBoxA
fun void hello_world
  logic
    call i32 user32.MessageBoxA
      ptr nullptr
      ptr \"Hello World!\"
      ptr \"title\"
      u32 0
";
#[test]
fn lexer_test1() {
    let tokens = lexer::analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let expect = "
Indent(0) Fun Void Id(\"hello_world\")
Indent(2) Logic
Indent(4) Call I32 Id(\"user32.MessageBoxA\")
Indent(6) Ptr Nullptr
Indent(6) Ptr Str(\"\\\"Hello World!\\\"\")
Indent(6) Ptr Str(\"\\\"title\\\"\")
Indent(6) U32 Int(\"0\") Eof
";
    assert_eq!(tokens.format(), expect);
}
#[test]
fn parser_test1() {
    let mut tokens = lexer::analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let ast = parser::parse(&mut tokens);
    let expect = "
Program
  Blocks
    [
      FunBlock
        Type { Void }
        Id(\"hello_world\")
        None
        Logics
          [
            CallLogic
              Type { I32 }
              Id(\"user32.MessageBoxA\")
              Args
                [
                  Arg { Type { Ptr } , Data { Nullptr } }
                  Arg { Type { Ptr } , Data { Str(\"\\\"Hello World!\\\"\") } }
                  Arg { Type { Ptr } , Data { Str(\"\\\"title\\\"\") } }
                  Arg { Type { U32 } , Data { Int(\"0\") } }
                ]
          ]
    ]
";
    assert_eq!(ast.format(), expect);
}
#[test]
fn format_test1() {
    let mut tokens = lexer::analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let ast = parser::parse(&mut tokens);
    let expect = "\
#[link(name=\"user32\")]
extern \"stdcall\" {
    fn MessageBoxA(_: i32, _: i32, _: i32, _: u32) -> i32;
}
fn hello_world() -> () {
    unsafe { MessageBoxA(0, \"Hello World!\".as_ptr() as i32, \"title\".as_ptr() as i32, 0) };
}
";
    assert_eq!(ast.format_for_rust(), expect);
}

#[cfg(test)]
const CODE2: &'static str = "
# Hello World with MessageBoxA with args
fun void hello_world
  args
    ptr msg
    ptr ttl
  logic
    call i32 user32.MessageBoxA
      ptr nullptr
      ptr \"Hello World!\"
      ptr \"title\"
      u32 0
";
#[test]
fn format_test2() {
    let mut tokens = lexer::analyze_tokens(CODE2.split('\n').map(|n| String::from(n)).collect());
    let ast = parser::parse(&mut tokens);
    let expect = "\
#[link(name=\"user32\")]
extern \"stdcall\" {
    fn MessageBoxA(_: i32, _: i32, _: i32, _: u32) -> i32;
}
fn hello_world(msg: i32, ttl: i32) -> () {
    unsafe { MessageBoxA(0, \"Hello World!\".as_ptr() as i32, \"title\".as_ptr() as i32, 0) };
}
";
    assert_eq!(ast.format_for_rust(), expect);
}