use super::{cons::CONS, tokenize::Token};

#[derive(Debug, Clone)]
pub enum CELL {
    ATOM(Token),
    PAIR(CONS),
}

impl CELL {
    pub fn cons(a: CELL, d: CELL) -> CELL {
        CELL::PAIR(CONS {
            x: Box::new(a),
            y: Box::new(d),
        })
    }
    pub fn uncons(s: CELL) -> (CELL, CELL) {
        match s {
            CELL::PAIR(CONS { x, y }) => (
                ::std::mem::replace(&mut *Box::leak(x), CELL::ATOM(Token::NIL)),
                ::std::mem::replace(&mut *Box::leak(y), CELL::ATOM(Token::NIL)),
            ),
            CELL::ATOM(s) => (CELL::ATOM(s), CELL::ATOM(Token::NIL)),
        }
    }
    pub fn atom(s: &CELL) -> bool {
        match &s {
            CELL::PAIR(CONS { x: _, y: _ }) => false,
            CELL::ATOM(_) => true,
        }
    }
}
