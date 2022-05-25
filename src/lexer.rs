#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // System
    Indent,
    Eof,
    // Symbol
    Fun,
    Args,
    Logic,
    Call,
    // Type
    Ptr,
    I32,
    U32,
    // Data
    Nullptr,
    Str(String),
    Int(String),
    Float(String),
    Id(String),
}
impl Token {
    fn from(w: &str) -> Self {
        use Token::*;
        match w {
            "fun" => Fun,
            "args" => Args,
            "logic" => Logic,
            "call" => Call,
            "ptr" => Ptr,
            "nullptr" => Nullptr,
            "i32" => I32,
            "u32" => U32,
            _ => {
                let mut chars = w.chars();
                if chars.nth(0).unwrap() == '\"' && chars.nth_back(0).unwrap() == '\"' {
                    Str(String::from(w))
                } else if is_integer_literal(w) {
                    Int(String::from(w))
                } else if is_float_literal(w) {
                    Float(String::from(w))
                } else {
                    Id(String::from(w))
                }
            }
        }
    }
}
fn is_integer_literal(trg: &str) -> bool {
    trg.parse::<i64>().is_ok() || trg.parse::<u64>().is_ok()
}
fn is_float_literal(trg: &str) -> bool {
    trg.parse::<f64>().is_ok()
}
#[derive(Debug, PartialEq)]
pub struct Tokens {
    tokens: Vec<Token>,
    idx: usize,
}
impl Tokens {
    pub fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, idx: 0 }
    }
    pub fn look(&self) -> Option<&Token> {
        self.tokens.get(self.idx)
    }
    pub fn consume(&mut self) -> Token {
        if self.idx >= self.tokens.len() {
            panic!("[Lexer error] Try to consume EOF.")
        }
        if cfg!(test) {
            use std::io::Write;
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            let msg = format!("consume '{:?}'\n", self.tokens[self.idx]);
            handle.write_all(msg.as_bytes()).unwrap();
        }
        self.idx += 1;
        self.tokens[self.idx - 1].clone()
    }
    pub fn consume_expect(&mut self, expect: Token) -> Token {
        if let Some(token) = self.look() {
            if std::mem::discriminant(token) == std::mem::discriminant(&expect) {
                self.consume()
            } else {
                panic!(
                    "[Lexer error] expected '{:?}', but found '{:?}'.",
                    expect, token
                );
            }
        } else {
            panic!("[Lexer error] expected '{:?}', but no token found.", expect);
        }
    }
}
/// A function to split line considering string literal
/// and get number of spaces that's at the top of the line.
fn split_line(trg: String) -> (Vec<String>, usize) {
    let mut v = Vec::new();
    let mut buf = String::new();
    let mut num_spaces = 0;
    let mut is_started = false;
    let mut is_in_word = false;
    let mut is_literal = false;
    for c in trg.chars() {
        if c == '#' {
            break;
        }
        if c == '\"' {
            is_literal = !is_literal;
        }
        if c == ' ' {
            if is_in_word {
                if is_literal {
                    buf.push(c);
                } else {
                    v.push(buf.clone());
                    is_in_word = false;
                }
            } else if !is_started {
                num_spaces += 1;
            }
            continue;
        }
        if !is_in_word {
            is_started = true;
            is_in_word = true;
            buf.clear();
        }
        buf.push(c);
    }
    if buf.len() > 0 {
        v.push(buf.clone());
    }
    (v, num_spaces)
}
pub fn analyze_tokens(lines: Vec<String>) -> Tokens {
    let mut v = Vec::new();
    let mut indent = 0;
    for line in lines {
        let (words, num_spaces) = split_line(line);
        if words.len() > 0 {
            if num_spaces != indent {
                v.push(Token::Indent);
            }
            indent = num_spaces;
        }
        for w in words {
            v.push(Token::from(&w));
        }
    }
    v.push(Token::Indent);
    v.push(Token::Eof);
    Tokens::from(v)
}
pub fn analyze_tokens_from_file(path: &String) -> Tokens {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };
    let reader = BufReader::new(File::open(path).unwrap());
    let lines = reader.lines().map(|n| n.unwrap()).collect();
    analyze_tokens(lines)
}
