use strum::{EnumCount, VariantArray};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray)]
pub enum PromotablePiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl Into<Piece> for PromotablePiece {
    fn into(self) -> Piece {
        self.into_piece()
    }
}

impl PromotablePiece {
    pub const fn from_piece(piece: Piece) -> Option<Self> {
        Some(match piece {
            Piece::Knight => Self::Knight,
            Piece::Bishop => Self::Bishop,
            Piece::Rook => Self::Rook,
            Piece::Queen => Self::Queen,
            _ => return None,
        })
    }
    pub const fn into_piece(self) -> Piece {
        match self {
            Self::Knight => Piece::Knight,
            Self::Bishop => Piece::Bishop,
            Self::Rook => Piece::Rook,
            Self::Queen => Piece::Queen,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray)]
pub enum CapturablePiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl Into<Piece> for CapturablePiece {
    fn into(self) -> Piece {
        self.into_piece()
    }
}

impl CapturablePiece {
    pub const fn from_piece(piece: Piece) -> Option<Self> {
        Some(match piece {
            Piece::Pawn => Self::Pawn,
            Piece::Knight => Self::Knight,
            Piece::Bishop => Self::Bishop,
            Piece::Rook => Self::Rook,
            Piece::Queen => Self::Queen,
            _ => return None,
        })
    }
    pub const fn into_piece(self) -> Piece {
        match self {
            Self::Pawn => Piece::Pawn,
            Self::Knight => Piece::Knight,
            Self::Bishop => Piece::Bishop,
            Self::Rook => Piece::Rook,
            Self::Queen => Piece::Queen,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray)]
pub enum CastlablePiece {
    King,
    Rook,
}

impl Into<Piece> for CastlablePiece {
    fn into(self) -> Piece {
        self.into_piece()
    }
}

impl CastlablePiece {
    pub const fn from_piece(piece: Piece) -> Option<Self> {
        Some(match piece {
            Piece::King => Self::King,
            Piece::Rook => Self::Rook,
            _ => return None,
        })
    }
    pub const fn into_piece(self) -> Piece {
        match self {
            Self::King => Piece::King,
            Self::Rook => Piece::Rook,
        }
    }
}
