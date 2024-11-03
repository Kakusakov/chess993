use bitflags::bitflags;
use crate::color::Color;
use crate::piece::Piece;
use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Castling {
    Kingside,
    Queenside,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CastlingRights: u8 {
        const WHITE_KING = 1 << 0;
        const WHITE_QUEEN = 1 << 1;
        const BLACK_KING = 1 << 2;
        const BLACK_QUEEN = 1 << 3;
        
        const WHITE = Self::WHITE_KING.bits() | Self::WHITE_QUEEN.bits();
        const BLACK = Self::BLACK_KING.bits() | Self::BLACK_QUEEN.bits();

        const KING = Self::WHITE_KING.bits() | Self::BLACK_KING.bits();
        const QUEEN = Self::WHITE_QUEEN.bits() | Self::BLACK_QUEEN.bits();

        const NONE = 0;
        const ALL = Self::WHITE.bits() | Self::BLACK.bits();
    }
}

impl From<Castling> for CastlingRights {
    fn from(value: Castling) -> Self {
        Self::from_castling(value)
    }
}

impl From<Color> for CastlingRights {
    fn from(value: Color) -> Self {
        Self::from_color(value)
    }
}

impl CastlingRights {
    pub const fn from_castling(castling: Castling) -> Self {
        match castling {
            Castling::Kingside => Self::KING,
            Castling::Queenside => Self::QUEEN,
        }
    }
    pub const fn from_color(color: Color) -> Self {
        match color {
            Color::White => Self::WHITE,
            Color::Black => Self::BLACK,
        }
    }
}
