use strum::EnumCount;

#[derive(Debug, Clone, Copy, EnumCount)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
