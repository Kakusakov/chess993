use crate::color::Color;
use crate::piece::{Piece, CapturablePiece, PromotablePiece};
use crate::bitboard::BitBoard;
use crate::square::{Rank, Square};
use strum::{EnumCount, VariantArray};

#[derive(Debug, Clone, Copy, Default)]
pub struct BitBoards {
    color: [BitBoard; Color::COUNT],
    piece: [BitBoard; Piece::COUNT],
}

impl BitBoards {
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
    pub fn color_mut(&mut self, color: Color) -> &mut BitBoard {
        &mut self.color[color as usize]
    }
    pub fn piece_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.piece[piece as usize]
    }
    pub fn move_piece(&mut self, color: Color, piece: Piece, from: Square, to: Square) {
        assert_eq!(self.piece_at(from), Some(piece));
        assert_eq!(self.color_at(from), Some(color));
        assert_eq!(self.piece_at(to), None);
        let update = BitBoard::from(from);
        *self.piece_mut(piece) ^= update;
        *self.color_mut(color) ^= update;
    }
    pub fn capture_piece(&mut self, color: Color, capture: CapturablePiece, sq: Square) {
        let piece = capture.into();
        assert_eq!(self.piece_at(sq), Some(piece));
        assert_eq!(self.color_at(sq), Some(color));
        let update = BitBoard::from(sq);
        *self.piece_mut(piece) ^= update;
        *self.color_mut(color) ^= update;
    }
    pub fn promote_piece(
        &mut self,
        color: Color,
        promotion: PromotablePiece,
        from: Square,
        to: Square
    ) {
        let piece_from = Piece::Pawn;
        let piece_to = promotion.into();
        assert_eq!(
            from.rank(),
            match color {
                Color::White => Rank::Seven,
                Color::Black => Rank::Two,
            }
        );
        assert_eq!(
            to.rank(),
            match color {
                Color::White => Rank::Eight,
                Color::Black => Rank::One,
            }
        );
        assert_eq!(self.piece_at(to), None);
        assert_eq!(self.piece_at(from), Some(piece_from));
        assert_eq!(self.color_at(from), Some(color));
        let update_from = BitBoard::from(from);
        *self.piece_mut(piece_from) ^= update_from;
        *self.color_mut(color) ^= update_from;
        let update_to = BitBoard::from(to);
        *self.piece_mut(piece_to) ^= update_to;
        *self.color_mut(color) ^= update_to;
    }
}
