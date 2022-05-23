use super::lexer::{Token::*, *};

fn error(place: &str, expect: &str, found: Option<&Token>) {
    if let Some(n) = found {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but '{:?}' found.",
                place, expect, n
            )
        );
    } else {
        panic!(
            "{}",
            format!(
                "[Parser error] {} : {} expected, but no token found.",
                place, expect
            )
        );
    }
}

// <Program> ::= <Block>$
pub fn parse(tokens: &mut Tokens) {
    parse_block(tokens);
    tokens.consume_expect(Eof);
}
// <Block> ::= fun <Function> (<Block> | "")
fn parse_block(tokens: &mut Tokens) {
    match tokens.look() {
        Some(Fun) => {
            tokens.consume();
            parse_function(tokens)
        }
        None => panic!("[Parser error] "),
        _ => panic!("[Parser error] Block"),
    }
}
// <Type> ::= ptr | i32 | u32
fn parse_type(tokens: &mut Tokens) {
    match tokens.look() {
        Some(Ptr) => tokens.consume(),
        Some(I32) => tokens.consume(),
        Some(U32) => tokens.consume(),
        n => error("Type", "type token", n),
    }
}
// <Data> ::= nullptr | str | int | float | id
fn parse_data(tokens: &mut Tokens) {
    match tokens.look() {
        Some(Nullptr) => tokens.consume(),
        Some(Str(_)) => tokens.consume(),
        Some(Int(_)) => tokens.consume(),
        Some(Float(_)) => tokens.consume(),
        Some(Id(_)) => tokens.consume(),
        n => error("Data", "data token", n),
    }
}
// <Function> ::= id
//  (indent args indent <Args> | "")
//  (indent logic indent <Logic> | "")
fn parse_function(tokens: &mut Tokens) {
    tokens.consume_expect(Id(String::new()));
    match tokens.look() {
        Some(Indent) => {
            tokens.consume();
            match tokens.look() {
                Some(Args) => tokens.consume(),
                Some(Logic) => {
                    tokens.consume();
                    tokens.consume_expect(Indent);
                    parse_logic(tokens);
                }
                n => error("Function", "'args' or 'logic'", n),
            }
        }
        _ => (),
    }
}
// <Logic> ::= call <Call>
fn parse_logic(tokens: &mut Tokens) {
    match tokens.look() {
        Some(Call) => {
            tokens.consume();
            parse_call(tokens);
        }
        n => error("Function", "'call'", n),
    }
}
// <Call> ::= id (indent <CallArgs> | "")
fn parse_call(tokens: &mut Tokens) {
    tokens.consume_expect(Id(String::new()));
    match tokens.look() {
        Some(Indent) => {
            tokens.consume();
            parse_callargs(tokens);
        }
        n => error("Call", "indent", n),
    }
}
// <CallArgs> ::= <Type> <Data> (indent | <CallArgs>)
fn parse_callargs(tokens: &mut Tokens) {
    parse_type(tokens);
    parse_data(tokens);
    match tokens.look() {
        Some(Indent) => tokens.consume(),
        _ => parse_callargs(tokens),
    }
}
