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
      i32 0
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
Indent(6) I32 Int(\"0\") Eof
";
    assert_eq!(tokens.format(), expect);
}
#[test]
fn parser_test1() {
    let mut tokens = lexer::analyze_tokens(CODE1.split('\n').map(|n| String::from(n)).collect());
    let ast = parser::parse(&mut tokens);
    let expect = "
Program {
  Blocks {
    [
      FunBlock {
        Type { Void }
        Id(\"hello_world\")
        Logics {
          [
            CallLogic {
              Type { I32 }
              Id(\"user32.MessageBoxA\")
              CallArgs {
                [
                  CallArg { Type { Ptr } , Data { Nullptr } }
                  CallArg { Type { Ptr } , Data { Str(\"\\\"Hello World!\\\"\") } }
                  CallArg { Type { Ptr } , Data { Str(\"\\\"title\\\"\") } }
                  CallArg { Type { I32 } , Data { Int(\"0\") } }
                ]
              }
            }
          ]
        }
      }
    ]
  }
}
";
    assert_eq!(ast.format(), expect);
}
