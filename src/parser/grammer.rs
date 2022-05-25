use super::*;

// <Program> ::= indent <Block> indent $
pub fn parse(tokens: &mut Tokens) -> Box<Cell> {
    let indent = tokens.consume_indent();
    let res = parse_block(tokens, &indent);
    let _ = tokens.consume_expect(Indent(0));
    let _ = tokens.consume_expect(Eof);
    res
}
// <Block> ::= fun <Function> (indent <Block> | "")
fn parse_block(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    match tokens.look() {
        Some(Fun) => Cell::box_pair(
            Cell::box_atom(tokens.consume()),
            parse_function(tokens, indent),
        ),
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
// <Function> ::= id (deferent_indent logic deferent_indent <Logic> | "")
fn parse_function(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let id = Cell::box_atom(tokens.consume_expect(Id(String::new())));
    // Get the indent of 'args' or 'logic'.
    // If it passes the match statement, there must be 'args' or 'logic'.
    let new_indent = match tokens.look() {
        Some(Indent(n)) if n > indent => tokens.consume_indent(),
        _ => return cell_list![id, Cell::box_nil()],
    };
    let logic = match tokens.look() {
        Some(Logic) => {
            let _ = tokens.consume();
            let n = tokens.consume_indent();
            if n <= new_indent {
                panic!(
                    "[Parser error] expected larger indent than {}, but found {} indent.",
                    new_indent, n
                );
            }
            parse_logic(tokens, &n)
        }
        n => error("Function", "'logic'", n),
    };
    Cell::box_pair(id, logic)
}
// <Logic> ::= call <Call> (same_indent <Logic> | "")
fn parse_logic(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let res = match tokens.look() {
        Some(Call) => Cell::box_pair(Cell::box_atom(tokens.consume()), parse_call(tokens, indent)),
        n => error("Logic", "'call'", n),
    };
    match tokens.look() {
        Some(Indent(n)) if n == indent => {
            let _ = tokens.consume();
            Cell::box_pair(res, parse_logic(tokens, indent))
        }
        _ => cell_list![res],
    }
}
// <Call> ::= id (deferent_indent <CallArgs> | "")
fn parse_call(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let id = Cell::box_atom(tokens.consume_expect(Id(String::new())));
    let new_indent_opt = match tokens.look() {
        Some(Indent(n)) if n > indent => Some(tokens.consume_indent()),
        _ => None,
    };
    if let Some(new_indent) = new_indent_opt {
        Cell::box_pair(id, parse_callargs(tokens, &new_indent))
    } else {
        cell_list![id]
    }
}
// <CallArgs> ::= <Type> <Data> (same_indent <CallArgs> | "")
fn parse_callargs(tokens: &mut Tokens, indent: &usize) -> Box<Cell> {
    let type_data = cell_list![parse_type(tokens), parse_data(tokens)];
    match tokens.look() {
        Some(Indent(n)) if n == indent => {
            let _ = tokens.consume();
            Cell::box_pair(type_data, parse_callargs(tokens, indent))
        }
        _ => cell_list![type_data],
    }
}
