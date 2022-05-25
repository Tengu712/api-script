use super::{lexer::*, parser::*};

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
  let ast = parse(&mut tokens);
  let expect = "
Fun Id(\"big_func\") 
  Call Id(\"4args_func\") 
    I32 Int(\"0\") 
    I32 Int(\"0\") 
    I32 Int(\"0\") 
    I32 Int(\"0\") 
  Call Id(\"2args_func\") 
    U32 Int(\"0\") 
    U32 Int(\"0\") 
";
  assert_eq!(ast.format(), expect);
}
