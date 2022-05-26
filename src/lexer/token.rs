#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // System
    Indent(usize),
    Eof,
    // Symbol
    Fun,
    Args,
    Logic,
    Call,
    // Type
    Void,
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
    pub fn from(w: &str) -> Self {
        use Token::*;
        match w {
            "fun" => Fun,
            "args" => Args,
            "logic" => Logic,
            "call" => Call,
            "void" => Void,
            "ptr" => Ptr,
            "nullptr" => Nullptr,
            "i32" => I32,
            "u32" => U32,
            _ => {
                if w.starts_with('\"') && w.ends_with('\"') {
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
