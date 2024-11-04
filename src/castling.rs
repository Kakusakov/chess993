use bitflags::bitflags;
use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    King,
    Queen,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct Castling: u8 {
        const WHITE_KING = 1 << 0;
        const WHITE_QUEEN = 1 << 1;
        const BLACK_KING = 1 << 2;
        const BLACK_QUEEN = 1 << 3;
        
        const WHITE = Self::WHITE_KING.bits() | Self::WHITE_QUEEN.bits();
        const BLACK = Self::BLACK_KING.bits() | Self::BLACK_QUEEN.bits();

        const KING = Self::WHITE_KING.bits() | Self::BLACK_KING.bits();
        const QUEEN = Self::WHITE_QUEEN.bits() | Self::BLACK_QUEEN.bits();
    }
}

impl From<CastlingSide> for Castling {
    fn from(value: CastlingSide) -> Self {
        Self::from_castling_side(value)
    }
}

impl From<Color> for Castling {
    fn from(value: Color) -> Self {
        Self::from_color(value)
    }
}

impl Castling {
    pub const fn from_castling_side(side: CastlingSide) -> Self {
        match side {
            CastlingSide::King => Self::KING,
            CastlingSide::Queen => Self::QUEEN,
        }
    }
    pub const fn from_color(color: Color) -> Self {
        match color {
            Color::White => Self::WHITE,
            Color::Black => Self::BLACK,
        }
    }
    pub fn with_rook_updated(self, color: Color, side: CastlingSide) -> Self {
        self & !(Castling::from(color) & Castling::from(side))
    }
    pub fn with_king_updated(self, color: Color) -> Self {
        self & !Castling::from(color)
    }
}
