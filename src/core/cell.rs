use super::{cons::Cons, tokenize::Token};

#[derive(Debug, Clone)]
pub enum Cell {
    ATOM(Token),
    PAIR(Cons),
}

impl Cell {
    pub fn cons(a: Cell, d: Cell) -> Cell {
        Cell::PAIR(Cons {
            x: Box::new(a),
            y: Box::new(d),
        })
    }
    pub fn uncons(s: Cell) -> (Cell, Cell) {
        match s {
            Cell::PAIR(Cons { x, y }) => (
                ::std::mem::replace(&mut *Box::leak(x), Cell::ATOM(Token::NIL)),
                ::std::mem::replace(&mut *Box::leak(y), Cell::ATOM(Token::NIL)),
            ),
            Cell::ATOM(s) => (Cell::ATOM(s), Cell::ATOM(Token::NIL)),
        }
    }
    pub fn atom(s: &Cell) -> bool {
        match &s {
            Cell::PAIR(Cons { x: _, y: _ }) => false,
            Cell::ATOM(_) => true,
        }
    }
}
