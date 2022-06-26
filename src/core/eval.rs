use super::{
    cell::{Token, CELL},
    tokenize::{atomev, eqev, s_append, s_pair, s_read},
};

pub fn eval(s: &str) -> CELL {
    s_eval(s_read(s), CELL::ATOM(Token::NIL))
}

fn s_eval(e: CELL, a: CELL) -> CELL {
    if CELL::atom(&e) {
        s_assoc(e, a)
    } else {
        let (ea, ed) = CELL::uncons(e);
        if CELL::atom(&ea) {
            match ea {
                CELL::ATOM(Token::QUOTE) => {
                    let (eda, _) = CELL::uncons(ed);
                    eda
                }
                CELL::ATOM(Token::ATOM) => {
                    let (eda, _) = CELL::uncons(ed);
                    let t = a.clone();
                    atomev(s_eval(eda, t))
                }
                CELL::ATOM(Token::EQ) => {
                    let (eda, edd) = CELL::uncons(ed);
                    let (edda, _) = CELL::uncons(edd);
                    let t1 = a.clone();
                    let t2 = a.clone();
                    eqev(s_eval(eda, t1), s_eval(edda, t2))
                }
                CELL::ATOM(Token::CAR) => {
                    let t = a.clone();
                    let (eda, _) = CELL::uncons(ed);
                    let (ra, _) = CELL::uncons(s_eval(eda, t));
                    ra
                }
                CELL::ATOM(Token::CDR) => {
                    let t = a.clone();
                    let (eda, _) = CELL::uncons(ed);
                    let (_, rd) = CELL::uncons(s_eval(eda, t));
                    rd
                }
                CELL::ATOM(Token::CONS) => {
                    let (eda, edd) = CELL::uncons(ed);
                    let (edda, _) = CELL::uncons(edd);
                    let t1 = a.clone();
                    let t2 = a.clone();
                    CELL::cons(s_eval(eda, t1), s_eval(edda, t2))
                }
                CELL::ATOM(Token::COND) => evcon(ed, a),
                _ => {
                    let t1 = a.clone();
                    let t2 = a.clone();
                    s_eval(CELL::cons(s_assoc(ea, t1), ed), t2)
                }
            }
        } else {
            let (eaa, ead) = CELL::uncons(ea);
            if eaa == CELL::ATOM(Token::LAMBDA) {
                let (eada, eadd) = CELL::uncons(ead);
                let (eadda, _) = CELL::uncons(eadd);
                let t1 = a.clone();
                let t2 = a.clone();
                s_eval(eadda, s_append(s_pair(eada, evlis(ed, t1)), t2))
            } else {
                CELL::ATOM(Token::NIL)
            }
        }
    }
}

fn s_assoc(x: CELL, y: CELL) -> CELL {
    let (ya, yd) = CELL::uncons(y);
    let (yaa, yad) = CELL::uncons(ya);
    let (yada, _) = CELL::uncons(yad);
    if yaa == x {
        yada
    } else {
        s_assoc(x, yd)
    }
}

fn evcon(c: CELL, a: CELL) -> CELL {
    let t1 = a.clone();
    let t2 = a.clone();
    let (ca, cd) = CELL::uncons(c);
    let (caa, cad) = CELL::uncons(ca);
    let (cada, _) = CELL::uncons(cad);
    if s_eval(caa, t1) == CELL::ATOM(Token::T) {
        s_eval(cada, t2)
    } else {
        evcon(cd, t2)
    }
}

fn evlis(m: CELL, a: CELL) -> CELL {
    if m == CELL::ATOM(Token::NIL) {
        CELL::ATOM(Token::NIL)
    } else {
        let t1 = a.clone();
        let t2 = a.clone();
        let (ma, md) = CELL::uncons(m);
        CELL::cons(s_eval(ma, t1), evlis(md, t2))
    }
}
