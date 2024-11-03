use crate::color::Color;
use crate::piece::Piece;
use crate::bitboard::BitBoard;
use crate::square::Square;
use strum::EnumCount;

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
    pub fn color_mut(&mut self, color: Color) -> &mut BitBoard {
        &mut self.color[color as usize]
    }
    pub fn piece_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.piece[piece as usize]
    }
    pub fn move_piece(&mut self, color: Color, piece: Piece, from: Square, to: Square) {
        self.remove_piece(color, piece, from);
        self.create_piece(color, piece, to);
    }
    pub fn remove_piece(&mut self, color: Color, piece: Piece, sq: Square) {
        assert!(self.color(color).has_square(sq));
        let update = BitBoard::from(sq);
        *self.piece_mut(piece) ^= update;
        *self.color_mut(color) ^= update;
    }
    pub fn create_piece(&mut self, color: Color, piece: Piece, sq: Square) {
        assert!(!self.color(color).has_square(sq));
        let update = BitBoard::from(sq);
        *self.piece_mut(piece) ^= update;
        *self.color_mut(color) ^= update;
    }
}
