use super::lexer::{Token::*, *};

fn error(place: &str, expect: &str, found: Option<&Token>) -> ! {
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

enum Cell {
    Atom(Token),
    Pair(Box<Cell>, Box<Cell>),
}
impl Cell {
    fn box_atom(token: Token) -> Box<Self> {
        Box::new(Self::Atom(token))
    }
    fn box_pair(x: Box<Cell>, y: Box<Cell>) -> Box<Self> {
        Box::new(Self::Pair(x, y))
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
            parse_function(tokens);
        }
        n => error("Block", "'fun'", n),
    }
}
// <Type> ::= ptr | i32 | u32
fn parse_type(tokens: &mut Tokens) -> Box<Cell> {
    match tokens.look() {
        Some(Ptr) | Some(I32) | Some(U32) => Cell::box_atom(tokens.consume()),
        n => error("Type", "type token", n),
    }
}
// <Data> ::= nullptr | str | int | float | id
fn parse_data(tokens: &mut Tokens) -> Box<Cell> {
    match tokens.look() {
        Some(Nullptr) | Some(Str(_)) | Some(Int(_)) | Some(Float(_)) | Some(Id(_)) => {
            Cell::box_atom(tokens.consume())
        }
        n => error("Data", "data token", n),
    }
}
// <Function> ::= id
//  (indent args indent <Args> | "")
//  (indent logic indent <Logic> | "")
fn parse_function(tokens: &mut Tokens) -> Box<Cell> {
    let id = Cell::box_atom(tokens.consume_expect(Id(String::new())));
    let args = match tokens.look() {
        Some(Indent) => {
            let _ = tokens.consume();
            match tokens.look() {
                Some(Args) => {
                    let _ = tokens.consume();
                    let _ = tokens.consume_expect(Indent);
                    Some(parse_logic(tokens))
                }
                Some(Logic) => None,
                n => error("Function", "'args' or 'logic'", n),
            }
        }
        _ => None,
    };
    let logic = match tokens.look() {
        Some(Indent) => {
            let _ = tokens.consume();
            let _ = tokens.consume_expect(Logic);
            let _ = tokens.consume_expect(Indent);
            Some(parse_logic(tokens))
        }
        _ => None,
    };
    let y = Cell::box_pair(, )
    id
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
