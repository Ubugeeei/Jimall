use super::super::interfaces::cell::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    QUOTE,
    EQ,
    PLUS,
    MINUS,
    MUL,
    DIVIDE,

    NUMBER(f64),
    SYMBOL(String),

    CAR,
    CDR,
    CONS,
    COND,

    LAMBDA,
    T,
    NIL,
}

pub fn parse(s: &str) -> Cell {
    let (rs, _) = parse_syntax(tokenize_to_strings(s));
    rs
}

fn tokenize_to_strings(s: &str) -> Vec<String> {
    let r: String = s
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", " ' ");
    r.split_whitespace().map(|x| x.to_string()).collect()
}

fn parse_syntax(mut s: Vec<String>) -> (Cell, Vec<String>) {
    let t = s.split_off(s.len() - 1);
    match &*t[0] {
        ")" => {
            let (r, ss) = parse_syntax0(Cell::ATOM(Token::NIL), s);
            parse_quote(r, ss)
        }
        _ => parse_quote(Cell::ATOM(tokenize_keyword(&t[0])), s),
    }
}

fn parse_syntax0(r: Cell, mut s: Vec<String>) -> (Cell, Vec<String>) {
    let mut t = s.split_off(s.len() - 1);
    match &*t[0] {
        "(" => (r, s),
        "." => {
            let (rr, rs) = parse_syntax(s);
            let (ca, _) = Cell::uncons(r);
            parse_syntax0(Cell::cons(rr, ca), rs)
        }
        _ => {
            s.append(&mut t);
            let (rr, rs) = parse_syntax(s);
            let c = Cell::cons(rr, r);
            parse_syntax0(c, rs)
        }
    }
}

fn parse_quote(x: Cell, mut s: Vec<String>) -> (Cell, Vec<String>) {
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

fn tokenize_keyword(token: &str) -> Token {
    match token {
        "cdr" => Token::CDR,
        "car" => Token::CAR,
        "cond" => Token::COND,
        "cons" => Token::CONS,
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
