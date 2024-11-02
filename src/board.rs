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
    pub fn color_mut(&mut self, color: Color) -> &mut BitBoard {
        &mut self.color[color as usize]
    }
    pub fn piece_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.piece[piece as usize]
    }
    pub fn move_piece(&mut self, color: Color, piece: Piece, from: Square, to: Square) {
        todo!()
    }
    pub fn remove_piece(&mut self, color: Color, piece: Piece, sq: Square) {
        todo!()
    }
    pub fn create_piece(&mut self, color: Color, piece: Piece, sq: Square) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    bitboards: BitBoards,
    en_passant: Option<Square>,
    ply_clock: u8,
    turn: Color,
}
