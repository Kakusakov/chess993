use strum::{EnumCount, VariantArray};

use crate::square::File;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EnPassant {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    None,
}

impl EnPassant {
    pub const fn none() -> Self {
        Self::None
    }
    pub const fn from_file(file: File) -> Self {
        match file {
            File::A => Self::A,
            File::B => Self::B,
            File::C => Self::C,
            File::D => Self::D,
            File::E => Self::E,
            File::F => Self::F,
            File::G => Self::G,
            File::H => Self::H,
        }
    }
    pub const fn file(self) -> Option<File> {
        Some(match self {
            EnPassant::A => File::A,
            EnPassant::B => File::B,
            EnPassant::C => File::C,
            EnPassant::D => File::D,
            EnPassant::E => File::E,
            EnPassant::F => File::F,
            EnPassant::G => File::G,
            EnPassant::H => File::H,
            EnPassant::None => return None,
        })
    }
}
