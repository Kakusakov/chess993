use strum::{EnumCount, VariantArray};
use constmuck::{Contiguous, contiguous};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount, VariantArray, Contiguous)]
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

impl File {
    pub const fn from_u8(value: u8) -> Option<Self> {
        contiguous::from_integer(value)
    }
}
