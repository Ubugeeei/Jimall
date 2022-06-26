use super::{
    super::{
        interfaces::cell::Cell,
        parser::parse::{parse, Token},
    },
    arithmetic::{evadd, evdiv, evsmul, evsub},
};

pub fn eval(s: &str) -> Cell {
    s_eval(parse(s), Cell::ATOM(Token::NIL))
}

fn s_eval(e: Cell, a: Cell) -> Cell {
    if Cell::atom(&e) {
        s_assoc(e, a)
    } else {
        let (ea, ed) = Cell::uncons(e);
        if Cell::atom(&ea) {
            match ea {
                Cell::ATOM(Token::QUOTE) => {
                    let (eda, _) = Cell::uncons(ed);
                    eda
                }
                Cell::ATOM(Token::EQ) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    let t1 = a.clone();
                    let t2 = a.clone();
                    eqev(s_eval(eda, t1), s_eval(edda, t2))
                }
                Cell::ATOM(Token::CAR) => {
                    let t = a.clone();
                    let (eda, _) = Cell::uncons(ed);
                    let (ra, _) = Cell::uncons(s_eval(eda, t));
                    ra
                }
                Cell::ATOM(Token::CDR) => {
                    let t = a.clone();
                    let (eda, _) = Cell::uncons(ed);
                    let (_, rd) = Cell::uncons(s_eval(eda, t));
                    rd
                }
                Cell::ATOM(Token::CONS) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    let t1 = a.clone();
                    let t2 = a.clone();
                    Cell::cons(s_eval(eda, t1), s_eval(edda, t2))
                }
                Cell::ATOM(Token::COND) => evcon(ed, a),

                //Four arithmetic operations
                // TODO: nested arithmetic
                Cell::ATOM(Token::PLUS) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    evadd(eda, edda)
                }
                Cell::ATOM(Token::MINUS) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    evsub(eda, edda)
                }
                Cell::ATOM(Token::MUL) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    evsmul(eda, edda)
                }
                Cell::ATOM(Token::DIVIDE) => {
                    let (eda, edd) = Cell::uncons(ed);
                    let (edda, _) = Cell::uncons(edd);
                    evdiv(eda, edda)
                }
                _ => {
                    let t1 = a.clone();
                    let t2 = a.clone();
                    s_eval(Cell::cons(s_assoc(ea, t1), ed), t2)
                }
            }
        } else {
            let (eaa, ead) = Cell::uncons(ea);
            if eaa == Cell::ATOM(Token::LAMBDA) {
                let (eada, eadd) = Cell::uncons(ead);
                let (eadda, _) = Cell::uncons(eadd);
                let t1 = a.clone();
                let t2 = a.clone();
                s_eval(eadda, s_append(s_pair(eada, evlis(ed, t1)), t2))
            } else {
                Cell::ATOM(Token::NIL)
            }
        }
    }
}

fn s_assoc(x: Cell, y: Cell) -> Cell {
    let (ya, yd) = Cell::uncons(y);
    let (yaa, yad) = Cell::uncons(ya);
    let (yada, _) = Cell::uncons(yad);
    if yaa == x {
        yada
    } else {
        s_assoc(x, yd)
    }
}

fn evcon(c: Cell, a: Cell) -> Cell {
    let t1 = a.clone();
    let t2 = a.clone();
    let (ca, cd) = Cell::uncons(c);
    let (caa, cad) = Cell::uncons(ca);
    let (cada, _) = Cell::uncons(cad);
    if s_eval(caa, t1) == Cell::ATOM(Token::T) {
        s_eval(cada, t2)
    } else {
        evcon(cd, t2)
    }
}

fn evlis(m: Cell, a: Cell) -> Cell {
    if m == Cell::ATOM(Token::NIL) {
        Cell::ATOM(Token::NIL)
    } else {
        let t1 = a.clone();
        let t2 = a.clone();
        let (ma, md) = Cell::uncons(m);
        Cell::cons(s_eval(ma, t1), evlis(md, t2))
    }
}

fn eqev(s1: Cell, s2: Cell) -> Cell {
    if s1 == s2 {
        Cell::ATOM(Token::T)
    } else {
        Cell::ATOM(Token::NIL)
    }
}

fn s_pair(x: Cell, y: Cell) -> Cell {
    if s_null(&x) || s_null(&y) {
        Cell::ATOM(Token::NIL)
    } else if !(Cell::atom(&x)) && !(Cell::atom(&y)) {
        let (xa, xd) = Cell::uncons(x);
        let (ya, yd) = Cell::uncons(y);
        Cell::cons(s_list(xa, ya), s_pair(xd, yd))
    } else {
        Cell::ATOM(Token::NIL)
    }
}

fn s_append(x: Cell, y: Cell) -> Cell {
    if s_null(&x) {
        y
    } else {
        let (a, d) = Cell::uncons(x);
        Cell::cons(a, s_append(d, y))
    }
}
fn s_null(x: &Cell) -> bool {
    x == &Cell::ATOM(Token::NIL)
}
fn s_list(x: Cell, y: Cell) -> Cell {
    Cell::cons(x, Cell::cons(y, Cell::ATOM(Token::NIL)))
}
