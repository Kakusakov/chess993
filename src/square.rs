use strum::{EnumCount, VariantArray};
use constmuck::{Contiguous, contiguous};

mod rank;
mod file;
mod pos_diag;
mod neg_diag;

pub use rank::Rank;
pub use file::File;
pub use pos_diag::PosDiag;
pub use neg_diag::NegDiag;

#[repr(u8)]
#[derive(Debug, Clone, Copy, EnumCount, VariantArray, Contiguous)]
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

impl Into<Rank> for Square {
    fn into(self) -> Rank {
        self.rank()
    }
}

impl Into<File> for Square {
    fn into(self) -> File {
        self.file()
    }
}

impl Into<PosDiag> for Square {
    fn into(self) -> PosDiag {
        self.pos_diag()
    }
}

impl Into<NegDiag> for Square {
    fn into(self) -> NegDiag {
        self.neg_diag()
    }
}

impl Square {
    pub const fn from_u8(value: u8) -> Option<Self> {
        contiguous::from_integer(value)
    }
    pub const fn straights(rank: Rank, file: File) -> Self {
        match Self::from_u8(rank as u8 * 8 + file as u8) {
            Some(result) => result,
            None => unreachable!()
        }
    }
    pub const fn diagonals(pos: PosDiag, neg: NegDiag) -> Self {
        let rank = match Rank::from_u8((neg as i8 + 7 + pos as i8) as u8 / 2) {
            Some(result) => result,
            None => unreachable!()
        };
        let file = match File::from_u8((neg as i8 + 7 - pos as i8) as u8 / 2) {
            Some(result) => result,
            None => unreachable!()
        };
        Self::straights(rank, file)
    }
    pub const fn rank(self) -> Rank {
        match Rank::from_u8(self as u8 / 8) {
            Some(result) => result,
            None => unreachable!()
        }
    }
    pub const fn file(self) -> File {
        match File::from_u8(self as u8 % 8) {
            Some(result) => result,
            None => unreachable!()
        }
    }
    pub const fn pos_diag(self) -> PosDiag {
        match PosDiag::from_i8(self.rank() as i8 - self.file() as i8) {
            Some(result) => result,
            None => unreachable!()
        }
    }
    pub const fn neg_diag(self) -> NegDiag {
        match NegDiag::from_i8(self.rank() as i8 + self.file() as i8 - 7) {
            Some(result) => result,
            None => unreachable!()
        }
    }
}
