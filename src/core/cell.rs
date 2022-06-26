use super::cons::CONS;

#[derive(Debug, Clone)]
pub enum CELL {
    ATOM(Token),
    PAIR(CONS),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    QUOTE,
    EQ,
    CAR,
    CDR,
    CONS,
    COND,
    LAMBDA,
    NIL,
    T,
    SYMBOL(String),
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
