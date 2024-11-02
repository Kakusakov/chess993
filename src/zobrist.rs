use crate::square::{File, Square};
use crate::color::Color;
use crate::piece::Piece;
use crate::castling::Castling;

pub struct Zobrist;

impl Zobrist {
    pub fn piece_on_square(sq: Square, color: Color, piece: Piece) -> u64 {
        todo!()
    }
    pub fn turn(color: Color) -> u64 {
        todo!()
    }
    pub fn castling(castling: Castling) -> Castling {
        todo!()
    }
    pub fn en_passant(file: File) -> u64 {
        todo!()
    }
}
