use enum_iterator::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

from_u8!(File);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

from_u8!(Rank);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

from_u8!(Square);

impl Square {
    pub const fn new(f: File, r: Rank) -> Self {
        Self::from_u8(f as u8 * 8 + r as u8)
    }
    pub const fn file(self) -> File {
        File::from_u8(self as u8 / 8)
    }
    pub const fn rank(self) -> Rank {
        Rank::from_u8(self as u8 % 8)
    }
}
