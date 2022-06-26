use super::cell::CELL;

#[derive(Debug, Clone)]
pub struct CONS {
    pub x: Box<CELL>,
    pub y: Box<CELL>,
}

#[allow(dead_code)]
impl PartialEq for CONS {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[allow(dead_code)]
impl PartialEq for CELL {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CELL::ATOM(s1) => match other {
                CELL::ATOM(s2) => s1 == s2,
                _ => false,
            },
            _ => false,
        }
    }
}
