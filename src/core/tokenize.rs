use super::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    QUOTE,
    EQ,
    CAR,
    CDR,
    Cons,
    COND,
    LAMBDA,
    NIL,
    T,
    PLUS,
    MINUS,
    MUL,
    DIVIDE,
    NUMBER(f64),
    SYMBOL(String),
}

pub fn tokenize_to_strings(s: &str) -> Vec<String> {
    let r: String = s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ");
    r.split_whitespace().map(|x| x.to_string()).collect()
}

pub fn tokenize_quote(x: Cell, mut s: Vec<String>) -> (Cell, Vec<String>) {
    if !s.is_empty() && s[s.len() - 1] == "'" {
        s.pop();
        (
            Cell::cons(
                Cell::ATOM(Token::QUOTE),
                Cell::cons(x, Cell::ATOM(Token::NIL)),
            ),
            s,
        )
    } else {
        (x, s)
    }
}

pub fn tokenize_syntax0(r: Cell, mut s: Vec<String>) -> (Cell, Vec<String>) {
    let mut t = s.split_off(s.len() - 1);
    match &*t[0] {
        "(" => (r, s),
        "." => {
            let (rr, rs) = tokenize_syntax(s);
            let (ca, _) = Cell::uncons(r);
            tokenize_syntax0(Cell::cons(rr, ca), rs)
        }
        _ => {
            s.append(&mut t);
            let (rr, rs) = tokenize_syntax(s);
            let c = Cell::cons(rr, r);
            tokenize_syntax0(c, rs)
        }
    }
}

pub fn tokenize_syntax(mut s: Vec<String>) -> (Cell, Vec<String>) {
    let t = s.split_off(s.len() - 1);
    match &*t[0] {
        ")" => {
            let (r, ss) = tokenize_syntax0(Cell::ATOM(Token::NIL), s);
            tokenize_quote(r, ss)
        }
        _ => tokenize_quote(Cell::ATOM(tokenize_keyword(&t[0])), s),
    }
}

fn tokenize_keyword(token: &str) -> Token {
    match token {
        "cdr" => Token::CDR,
        "car" => Token::CAR,
        "cond" => Token::COND,
        "cons" => Token::Cons,
        "eq" => Token::EQ,
        "lambda" => Token::LAMBDA,
        "nil" => Token::NIL,
        "quote" => Token::QUOTE,
        "t" => Token::T,
        "+" => Token::PLUS,
        "-" => Token::MINUS,
        "*" => Token::MUL,
        "/" => Token::DIVIDE,
        _ => {
            let t = token.parse::<f64>();
            match t {
                Ok(t) => Token::NUMBER(t),
                Err(_) => Token::SYMBOL(String::from(token)),
            }
        }
    }
}

pub fn s_read(s: &str) -> Cell {
    let (rs, _) = tokenize_syntax(tokenize_to_strings(s));
    rs
}

pub fn eqev(s1: Cell, s2: Cell) -> Cell {
    if s1 == s2 {
        Cell::ATOM(Token::T)
    } else {
        Cell::ATOM(Token::NIL)
    }
}

pub fn s_null(x: &Cell) -> bool {
    x == &Cell::ATOM(Token::NIL)
}

pub fn s_append(x: Cell, y: Cell) -> Cell {
    if s_null(&x) {
        y
    } else {
        let (a, d) = Cell::uncons(x);
        Cell::cons(a, s_append(d, y))
    }
}

pub fn s_list(x: Cell, y: Cell) -> Cell {
    Cell::cons(x, Cell::cons(y, Cell::ATOM(Token::NIL)))
}

pub fn s_pair(x: Cell, y: Cell) -> Cell {
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
