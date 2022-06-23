use super::{
    automata::{dfa::*, nfa::*},
    regex::*,
};

#[derive(Clone, Debug)]
pub enum Token {
    // System
    Space,
    Newline,
    Indent(usize),
    // Symbol
    Fun,
    Args,
    Logic,
    Call,
    // Type
    Void,
    Ptr,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    // Data
    Nullptr,
    Str(String),
    Int(String),
    Float(String),
    Id(String),
}

pub fn parse_tokens(src: Vec<u8>) -> Vec<Token> {
    let mut nfas = Vec::new();
    push(&mut nfas, "  *", Token::Space);
    push(&mut nfas, "\n", Token::Newline);
    push(&mut nfas, ":(0|1|2|3|4|5|6|7|8|9)*:", Token::Indent(0));
    push(&mut nfas, "fun", Token::Fun);
    push(&mut nfas, "args", Token::Args);
    push(&mut nfas, "logic", Token::Logic);
    push(&mut nfas, "call", Token::Call);
    push(&mut nfas, "void", Token::Void);
    push(&mut nfas, "ptr", Token::Ptr);
    push(&mut nfas, "i8", Token::I8);
    push(&mut nfas, "i16", Token::I16);
    push(&mut nfas, "i32", Token::I32);
    push(&mut nfas, "i64", Token::I64);
    push(&mut nfas, "u8", Token::U8);
    push(&mut nfas, "u16", Token::U16);
    push(&mut nfas, "u32", Token::U32);
    push(&mut nfas, "u64", Token::U64);
    push(&mut nfas, "nullptr", Token::Nullptr);
    push(&mut nfas, "\"_\"", Token::Str(String::new()));
    push(
        &mut nfas,
        "(0|1|2|3|4|5|6|7|8|9)*",
        Token::Int(String::new()),
    );
    push(
        &mut nfas,
        "(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*.(0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*",
        Token::Float(String::new()),
    );
    push(
        &mut nfas,
        "\
        (a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z\
         A|B|C|D|E|F|G|H|I|J|K|L|M|M|O|P|Q|R|S|T|U|V|W|X|Y|Z)\
        (a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z\
         A|B|C|D|E|F|G|H|I|J|K|L|M|M|O|P|Q|R|S|T|U|V|W|X|Y|Z\
         0|1|2|3|4|5|6|7|8|9|\\_)*\
        ",
        Token::Id(String::new()),
    );
    let mut itr = nfas.into_iter();
    let mut nfa = itr.next().unwrap();
    for i in itr {
        nfa = nfa.connect(&i);
    }
    let dfa = DFAutomata::from(&nfa);
    let mut itr = src.into_iter().peekable();
    let mut tokens = Vec::new();
    while let Some(n) = dfa.start_with(&mut itr) {
        match n {
            (Token::Indent(_), n) => tokens.push(Token::Indent(take_indent(n))),
            (Token::Str(_), n) => tokens.push(Token::Str(n)),
            (Token::Int(_), n) => tokens.push(Token::Int(n)),
            (Token::Float(_), n) => tokens.push(Token::Float(n)),
            (Token::Id(_), n) => tokens.push(Token::Id(n)),
            (Token::Space, _) | (Token::Newline, _) => (),
            (l, _) => tokens.push(l),
        }
    }
    tokens
}
fn push(nfas: &mut Vec<NFAutomata<Token>>, re_str: &'static str, label: Token) {
    nfas.push(parse_nfa(parse_regex(String::from(re_str)), label));
}
fn take_indent(trg: String) -> usize {
    let mut chars = trg.chars();
    chars.next();
    chars.next_back();
    chars.as_str().parse().unwrap()
}
