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
