use constmuck::{Contiguous, contiguous};
use strum::{EnumCount, VariantArray};

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray, Contiguous)]
pub enum PosDiag {
    H1H1 = -7,
    G1H2,
    F1H3,
    E1H4,
    D1H5,
    C1H6,
    B1H7,
    A1H8,
    A2G8,
    A3F8,
    A4E8,
    A5D8,
    A6C8,
    A7B8,
    A8A8,
}

impl PosDiag {
    pub const fn from_i8(value: i8) -> Option<Self> {
        contiguous::from_integer(value)
    }
}
