use aslp::lexer::*;

pub fn compile(rlex: &RuntimeLexer, src: Vec<u8>) -> Vec<Token> {
    let mut res = Vec::new();
    let mut itr = src.into_iter().peekable();
    while let Some((t, s)) = rlex.start_with(&mut itr) {
        match t {
            Token::Space => (),
            Token::Indent(_) => res.push(Token::Indent(take_indent(s))),
            Token::Str(_) => res.push(Token::Str(s)),
            Token::Int(_) => res.push(Token::Int(s)),
            Token::Float(_) => res.push(Token::Float(s)),
            Token::Id(_) => res.push(Token::Id(s)),
            _ => res.push(t),
        }
    }
    res
}
fn take_indent(trg: String) -> usize {
    let mut chars = trg.chars();
    let _ = chars.next();
    let _ = chars.next_back();
    chars.as_str().parse().unwrap()
}
