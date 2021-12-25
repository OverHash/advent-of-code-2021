use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, PartialEq)]
pub enum Rotation {
    Right,
    Down,
    None,
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Rotation::Right => write!(f, ">"),
            Rotation::Down => write!(f, "v"),
            Rotation::None => write!(f, "."),
        }
    }
}
