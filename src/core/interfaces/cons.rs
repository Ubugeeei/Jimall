use super::cell::Cell;

#[derive(Debug, Clone)]
pub struct Cons {
    pub x: Box<Cell>,
    pub y: Box<Cell>,
}

impl PartialEq for Cons {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Cell::ATOM(s1) => match other {
                Cell::ATOM(s2) => s1 == s2,
                _ => false,
            },
            _ => false,
        }
    }
}
