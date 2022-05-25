use super::{cell::*, *};

// <Program> ::= <Block>$
pub fn parse(tokens: &mut Tokens) -> Box<Cell> {
    let res = parse_block(tokens);
    let _ = tokens.consume_expect(Eof);
    res
}
// <Block> ::= fun <Function> (<Block> | "")
fn parse_block(tokens: &mut Tokens) -> Box<Cell> {
    match tokens.look() {
        Some(Fun) => {
            let fun = Cell::box_atom(tokens.consume());
            let fun_body = parse_function(tokens);
            Cell::box_pair(fun, fun_body)
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
// <Function> ::= id indent(2) logic <Logic>
fn parse_function(tokens: &mut Tokens) -> Box<Cell> {
    let id = Cell::box_atom(tokens.consume_expect(Id(String::new())));
    let _ = tokens.consume_expect(Indent);
    let logic = match tokens.look() {
        Some(Logic) => {
            let _ = tokens.consume();
            let _ = tokens.consume_expect(Indent);
            parse_logic(tokens)
        }
        _ => Cell::box_nil(),
    };
    Cell::box_pair(id, Cell::box_pair(args, logic))
}
// <Logic> ::= call <Call> (indent <Logic> | "")
fn parse_logic(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let res = match tokens.look() {
        Some(Call) => {
            let call = Cell::box_atom(tokens.consume());
            let call_body = parse_call(tokens);
            Cell::box_pair(call, call_body)
        }
        n => error("Logic", "'call'", n),
    };
    match tokens.look() {
        Some(Indent(n)) if n == indent => {
            let _ = tokens.consume();
            Cell::box_pair(res, parse_logic(tokens, indent))
        }
        _ => Cell::box_pair(res, Cell::box_nil()),
    }
}
// <Call> ::= id (indent <CallArgs> | "")
fn parse_call(tokens: &mut Tokens) -> Box<Cell> {
    let id = Cell::box_atom(tokens.consume_expect(Id(String::new())));
    match tokens.look() {
        Some(Indent) => {
            let _ = tokens.consume();
            Cell::box_pair(id, parse_callargs(tokens))
        }
        _ => Cell::box_pair(id, Cell::box_nil()),
    }
}
// <CallArgs> ::= <Type> <Data> (indent <CallArgs> | "")
fn parse_callargs(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let type_data = cell_list![parse_type(tokens), parse_data(tokens)];
    match tokens.look() {
        Some(Indent(n)) if n == indent => {
            let _ = tokens.consume();
            Cell::box_pair(type_data, parse_callargs(tokens, indent))
        }
        _ => Cell::box_pair(type_data, Cell::box_nil()),
    }
}
