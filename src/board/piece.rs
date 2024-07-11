use enum_iterator::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
