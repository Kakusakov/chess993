use enum_iterator::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Color {
    Black,
    White,
}

from_u8!(Color);

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
