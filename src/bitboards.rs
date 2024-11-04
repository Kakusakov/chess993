use crate::color::Color;
use crate::piece::{Piece, CapturablePiece, PromotablePiece};
use crate::bitboard::BitBoard;
use crate::square::{Rank, Square};
use strum::{EnumCount, VariantArray};

#[derive(Debug, Clone, Copy, Default)]
pub struct Bitboards {
    color: [BitBoard; Color::COUNT],
    piece: [BitBoard; Piece::COUNT],
}

impl Bitboards {
    pub const fn color(&self, color: Color) -> &BitBoard {
        &self.color[color as usize]
    }
    pub const fn piece(&self, piece: Piece) -> &BitBoard {
        &self.piece[piece as usize]
    }
    pub const fn occupance(&self) -> BitBoard {
        let white = *self.color(Color::White);
        let black = *self.color(Color::Black);
        white.bitor(black)
    }
    pub const fn color_at(&self, sq: Square) -> Option<Color> {
        let mut i = 0;
        while i < Color::COUNT {
            let color = Color::VARIANTS[i];
            if self.color(color).has_square(sq) {
                return Some(color)
            }
            i += 1;
        }
        None
    }
    pub const fn piece_at(&self, sq: Square) -> Option<Piece> {
        let mut i = 0;
        while i < Piece::COUNT {
            let piece = Piece::VARIANTS[i];
            if self.piece(piece).has_square(sq) {
                return Some(piece)
            }
            i += 1;
        }
        None
    }
    pub fn color_mut(&mut self, color: Color) -> &mut BitBoard {
        &mut self.color[color as usize]
    }
    pub fn piece_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.piece[piece as usize]
    }
    pub fn update(&mut self, color: Color, piece: Piece, bitboard: BitBoard) {
        *self.color_mut(color) ^= bitboard;
        *self.piece_mut(piece) ^= bitboard;
    }
}
