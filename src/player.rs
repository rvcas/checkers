use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    Red,
    White,
}

impl Player {
    pub fn is_red(&self) -> bool {
        match self {
            Self::Red => true,
            Self::White => false,
        }
    }

    pub fn is_white(&self) -> bool {
        match self {
            Self::Red => false,
            Self::White => true,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "red"),
            Self::White => write!(f, "white"),
        }
    }
}
