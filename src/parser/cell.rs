use super::*;

pub struct Cons(pub Box<Cell>, pub Box<Cell>);
pub enum Cell {
    Nil,
    Atom(Token),
    Pair(Cons),
}
impl Cell {
    pub(super) fn box_nil() -> Box<Self> {
        Box::new(Self::Nil)
    }
    pub(super) fn box_atom(token: Token) -> Box<Self> {
        Box::new(Self::Atom(token))
    }
    pub(super) fn box_pair(x: Box<Cell>, y: Box<Cell>) -> Box<Self> {
        Box::new(Self::Pair(Cons(x,y)))
    }
}
macro_rules! cell_list {
    [$($x:expr),+] => {
        {
            let mut list = Cons(Cell::box_nil(), Cell::box_nil());
            $(list = Cons($x, Cell::Pair(list_rev));)+
            list_rev
        }
    };
}