use super::cell::{Token, CELL};

pub fn s_lex(s: &str) -> Vec<String> {
    let r: String = s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ");
    r.split_whitespace().map(|x| x.to_string()).collect()
}

pub fn s_quote(x: CELL, mut s: Vec<String>) -> (CELL, Vec<String>) {
    if !s.is_empty() && s[s.len() - 1] == "'" {
        s.pop();
        (
            CELL::cons(
                CELL::ATOM(Token::QUOTE),
                CELL::cons(x, CELL::ATOM(Token::NIL)),
            ),
            s,
        )
    } else {
        (x, s)
    }
}

pub fn s_syn0(r: CELL, mut s: Vec<String>) -> (CELL, Vec<String>) {
    let mut t = s.split_off(s.len() - 1);
    match &*t[0] {
        "(" => (r, s),
        "." => {
            let (rr, rs) = s_syn(s);
            let (ca, _) = CELL::uncons(r);
            s_syn0(CELL::cons(rr, ca), rs)
        }
        _ => {
            s.append(&mut t);
            let (rr, rs) = s_syn(s);
            let c = CELL::cons(rr, r);
            s_syn0(c, rs)
        }
    }
}

pub fn s_syn(mut s: Vec<String>) -> (CELL, Vec<String>) {
    let t = s.split_off(s.len() - 1);
    if t[0] == ")" {
        let (r, ss) = s_syn0(CELL::ATOM(Token::NIL), s);
        s_quote(r, ss)
    } else {
        s_quote(CELL::ATOM(Token::SYMBOL(String::from(&t[0]))), s)
    }
}

pub fn s_read(s: &str) -> CELL {
    let (rs, _) = s_syn(s_lex(s));
    rs
}

pub fn eqev(s1: CELL, s2: CELL) -> CELL {
    if s1 == s2 {
        CELL::ATOM(Token::T)
    } else {
        CELL::ATOM(Token::NIL)
    }
}

pub fn atomev(s: CELL) -> CELL {
    if CELL::atom(&s) {
        CELL::ATOM(Token::T)
    } else {
        CELL::ATOM(Token::NIL)
    }
}

pub fn s_null(x: &CELL) -> bool {
    x == &CELL::ATOM(Token::NIL)
}

pub fn s_append(x: CELL, y: CELL) -> CELL {
    if s_null(&x) {
        y
    } else {
        let (a, d) = CELL::uncons(x);
        CELL::cons(a, s_append(d, y))
    }
}

pub fn s_list(x: CELL, y: CELL) -> CELL {
    CELL::cons(x, CELL::cons(y, CELL::ATOM(Token::NIL)))
}

pub fn s_pair(x: CELL, y: CELL) -> CELL {
    if s_null(&x) || s_null(&y) {
        CELL::ATOM(Token::NIL)
    } else if !(CELL::atom(&x)) && !(CELL::atom(&y)) {
        let (xa, xd) = CELL::uncons(x);
        let (ya, yd) = CELL::uncons(y);
        CELL::cons(s_list(xa, ya), s_pair(xd, yd))
    } else {
        CELL::ATOM(Token::NIL)
    }
}
