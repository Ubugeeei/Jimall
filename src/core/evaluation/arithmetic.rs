use super::super::{interfaces::cell::Cell, parser::parse::Token};

pub fn evadd(c: Cell, a: Cell) -> Cell {
    match c {
        Cell::ATOM(Token::NUMBER(i)) => match a {
            Cell::ATOM(Token::NUMBER(j)) => Cell::ATOM(Token::NUMBER(i + j)),
            _ => {
                panic!("evadd: type error");
            }
        },
        _ => panic!("evadd: type error"),
    }
}

pub fn evsub(c: Cell, a: Cell) -> Cell {
    match c {
        Cell::ATOM(Token::NUMBER(i)) => match a {
            Cell::ATOM(Token::NUMBER(j)) => Cell::ATOM(Token::NUMBER(i - j)),
            _ => {
                panic!("evsub: type error");
            }
        },
        _ => panic!("evsub: type error"),
    }
}

pub fn evsmul(c: Cell, a: Cell) -> Cell {
    match c {
        Cell::ATOM(Token::NUMBER(i)) => match a {
            Cell::ATOM(Token::NUMBER(j)) => Cell::ATOM(Token::NUMBER(i * j)),
            _ => {
                panic!("evsmul: type error");
            }
        },
        _ => panic!("evsmul: type error"),
    }
}

pub fn evdiv(c: Cell, a: Cell) -> Cell {
    match c {
        Cell::ATOM(Token::NUMBER(i)) => match a {
            Cell::ATOM(Token::NUMBER(j)) => Cell::ATOM(Token::NUMBER(i / j)),
            _ => {
                panic!("evdiv: type error");
            }
        },
        _ => panic!("evdiv: type error"),
    }
}
