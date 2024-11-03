use crate::bitboards::BitBoards;
use crate::square::Square;
use crate::color::Color;
use crate::castling::Castling;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    bitboards: BitBoards,
    turn: Color,
    castling: Castling,
    en_passant: Option<Square>,
    halfmove_clock: u8,
}
