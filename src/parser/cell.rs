use super::*;

#[derive(Debug)]
pub enum Cell {
    Nil,
    Atom(Token),
    Pair(Box<Cell>, Box<Cell>),
}
impl Cell {
    pub(super) fn box_nil() -> Box<Self> {
        Box::new(Self::Nil)
    }
    pub(super) fn box_atom(token: Token) -> Box<Self> {
        Box::new(Self::Atom(token))
    }
    pub(super) fn box_pair(x: Box<Cell>, y: Box<Cell>) -> Box<Self> {
        Box::new(Self::Pair(x,y))
    }
}
macro_rules! cell_list {
    [$($x:expr),*] => {
        {
            let mut v = Vec::<Box<Cell>>::new();
            $(v.push($x);)*
            let mut list = Cell::box_nil();
            for i in v.into_iter().rev() {
                list = Cell::box_pair(i, list);
            }
            list
        }
    };
}