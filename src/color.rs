use std::ops::Not;
use constmuck::{Contiguous, contiguous};
use strum::{EnumCount, VariantArray};

#[repr(u8)]
#[derive(Debug, Clone, Copy, EnumCount, VariantArray, Contiguous)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::not(self)
    }
}

impl Color {
    pub const fn from_u8(value: u8) -> Option<Self> {
        contiguous::from_integer(value)
    }
    pub const fn not(self) -> Self {
        match Self::from_u8(self as u8 ^ 0x1) {
            Some(result) => result,
            None => unreachable!(),
        }
    }
}
