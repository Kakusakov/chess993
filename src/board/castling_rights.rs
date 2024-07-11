use bitflags::*;
use crate::board::color::*;

bitflags! {
    pub struct CastlingRights: u8 {
        const WK = 1 << 0;
        const WQ = 1 << 1;
        const BK = 1 << 2;
        const BQ = 1 << 3;

        const WHITE = Self::WK.bits() | Self::WQ.bits();
        const BLACK = Self::BK.bits() | Self::BQ.bits();
    }
}

impl CastlingRights {
    pub fn king(color: Color) -> Self {
        match color {
            Color::White => Self::WK,
            Color::Black => Self::BK,
        }
    }
    pub fn queen(color: Color) -> Self {
        match color {
            Color::White => Self::WQ,
            Color::Black => Self::BQ,
        }
    }
    pub fn both(color: Color) -> Self {
        match color {
            Color::White => Self::WHITE,
            Color::Black => Self::BLACK,
        }
    }
}