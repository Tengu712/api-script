use super::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Character(u8),
    OpAlter,
    OpStar,
    LParen,
    RParen,
    Eps,
    Eof,
}
impl Token {
    pub fn from_restr(restr: String) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        let mut flag = false;
        for c in restr.into_bytes() {
            match c as char {
                _ if flag => {
                    flag = false;
                    tokens.push_back(Token::Character(c));
                }
                '\\' => {
                    flag = true;
                }
                '|' => tokens.push_back(Token::OpAlter),
                '*' => tokens.push_back(Token::OpStar),
                '(' => tokens.push_back(Token::LParen),
                ')' => tokens.push_back(Token::RParen),
                '$' => tokens.push_back(Token::Eps),
                _ => tokens.push_back(Token::Character(c)),
            }
        }
        tokens.push_back(Token::Eof);
        tokens
    }
}

fn consume_expect(tokens: &mut VecDeque<Token>, expected: Token) {
    match tokens.pop_front() {
        Some(t) if t == expected => (),
        n => panic!("Syntax error: {:?} found, but expected {:?}.", n, expected),
    }
}
/// <expr> ::= <subexpr>Eof
pub fn expr(tokens: &mut VecDeque<Token>) -> Regex {
    let n = subexpr(tokens);
    consume_expect(tokens, Token::Eof);
    n
}
/// <subexpr> ::= <seq> ( '|'<subexpr> | eps )
fn subexpr(tokens: &mut VecDeque<Token>) -> Regex {
    let n = seq(tokens);
    match tokens.front() {
        Some(Token::OpAlter) => {
            consume_expect(tokens, Token::OpAlter);
            Alter::new_box(n, subexpr(tokens))
        }
        _ => n,
    }
}
/// <seq> ::= <subseq> | Eps
fn seq(tokens: &mut VecDeque<Token>) -> Regex {
    match tokens.front() {
        Some(Token::Eps) => {
            consume_expect(tokens, Token::Eps);
            LChar::new_box(0)
        }
        _ => subseq(tokens),
    }
}
/// <subseq> ::= <star> ( <subseq> | eps )
fn subseq(tokens: &mut VecDeque<Token>) -> Regex {
    let n = star(tokens);
    match tokens.front() {
        Some(Token::LParen) | Some(Token::Character(_)) => Concat::new_box(n, subseq(tokens)),
        _ => n,
    }
}
/// <star> ::= <factor> ( '*' | eps )
fn star(tokens: &mut VecDeque<Token>) -> Regex {
    let n = factor(tokens);
    match tokens.front() {
        Some(Token::OpStar) => {
            consume_expect(tokens, Token::OpStar);
            Star::new_box(n)
        }
        _ => n,
    }
}
/// <factor> ::= '('<subexpr>')' | Character
fn factor(tokens: &mut VecDeque<Token>) -> Regex {
    match tokens.front() {
        Some(Token::LParen) => {
            consume_expect(tokens, Token::LParen);
            let n = subexpr(tokens);
            consume_expect(tokens, Token::RParen);
            n
        }
        Some(Token::Character(c)) => {
            let cloned = c.clone();
            consume_expect(tokens, Token::Character(cloned));
            LChar::new_box(cloned)
        }
        n => panic!(
            "Syntax error in factor: {:?} found, but expected '(' or Charactor.",
            n
        ),
    }
}
