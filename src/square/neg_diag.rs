use constmuck::{Contiguous, contiguous};
use strum::{EnumCount, VariantArray};

#[repr(i8)]
#[derive(Debug, Clone, Copy, EnumCount, VariantArray, Contiguous)]
pub enum NegDiag {
    A1A1 = -7,
    A2B1,
    A3C1,
    A4D1,
    A5E1,
    A6F1,
    A7G1,
    A8H1,
    B8H2,
    C8H3,
    D8H4,
    E8H5,
    F8H6,
    G8H7,
    H8H8,
}

impl NegDiag {
    pub const fn from_i8(value: i8) -> Option<Self> {
        contiguous::from_integer(value)
    }
}
