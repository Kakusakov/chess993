use crate::board::square::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Self(u64::MIN);
    pub const FILLED: Self = Self(u64::MAX);
}

impl Bitboard {
    pub const fn from_file(f: File) -> Self {
        Self(0x0101_0101_0101_0101 << (f as u8))
    }

    pub const A_FILE: Self = Self::from_file(File::A);
    pub const B_FILE: Self = Self::from_file(File::B);
    pub const C_FILE: Self = Self::from_file(File::C);
    pub const D_FILE: Self = Self::from_file(File::D);
    pub const E_FILE: Self = Self::from_file(File::E);
    pub const F_FILE: Self = Self::from_file(File::F);
    pub const G_FILE: Self = Self::from_file(File::G);
    pub const H_FILE: Self = Self::from_file(File::H);
}

impl std::convert::From<File> for Bitboard {
    fn from(value: File) -> Self {
        Self::from_file(value)
    }
}

impl Bitboard {
    pub const fn from_rank(r: Rank) -> Self {
        Self(0xFF << (r as u8 * 8))
    }

    pub const RANK_1: Self = Self::from_rank(Rank::R1);
    pub const RANK_2: Self = Self::from_rank(Rank::R2);
    pub const RANK_3: Self = Self::from_rank(Rank::R3);
    pub const RANK_4: Self = Self::from_rank(Rank::R4);
    pub const RANK_5: Self = Self::from_rank(Rank::R5);
    pub const RANK_6: Self = Self::from_rank(Rank::R6);
    pub const RANK_7: Self = Self::from_rank(Rank::R7);
    pub const RANK_8: Self = Self::from_rank(Rank::R8);
}

impl std::convert::From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        Self::from_rank(value)
    }
}

impl Bitboard {
    pub const fn from_square(sq: Square) -> Self {
        Self(0x1 << (sq as u8))
    }
    
    pub const A1: Self = Self::from_square(Square::A1); 
    pub const B1: Self = Self::from_square(Square::B1); 
    pub const C1: Self = Self::from_square(Square::C1); 
    pub const D1: Self = Self::from_square(Square::D1); 
    pub const E1: Self = Self::from_square(Square::E1); 
    pub const F1: Self = Self::from_square(Square::F1); 
    pub const G1: Self = Self::from_square(Square::G1); 
    pub const H1: Self = Self::from_square(Square::H1);
    
    pub const A2: Self = Self::from_square(Square::A2); 
    pub const B2: Self = Self::from_square(Square::B2); 
    pub const C2: Self = Self::from_square(Square::C2); 
    pub const D2: Self = Self::from_square(Square::D2); 
    pub const E2: Self = Self::from_square(Square::E2); 
    pub const F2: Self = Self::from_square(Square::F2); 
    pub const G2: Self = Self::from_square(Square::G2); 
    pub const H2: Self = Self::from_square(Square::H2);
    
    pub const A3: Self = Self::from_square(Square::A3); 
    pub const B3: Self = Self::from_square(Square::B3); 
    pub const C3: Self = Self::from_square(Square::C3); 
    pub const D3: Self = Self::from_square(Square::D3); 
    pub const E3: Self = Self::from_square(Square::E3); 
    pub const F3: Self = Self::from_square(Square::F3); 
    pub const G3: Self = Self::from_square(Square::G3); 
    pub const H3: Self = Self::from_square(Square::H3);
    
    pub const A4: Self = Self::from_square(Square::A4); 
    pub const B4: Self = Self::from_square(Square::B4); 
    pub const C4: Self = Self::from_square(Square::C4); 
    pub const D4: Self = Self::from_square(Square::D4); 
    pub const E4: Self = Self::from_square(Square::E4); 
    pub const F4: Self = Self::from_square(Square::F4); 
    pub const G4: Self = Self::from_square(Square::G4); 
    pub const H4: Self = Self::from_square(Square::H4);
    
    pub const A5: Self = Self::from_square(Square::A5); 
    pub const B5: Self = Self::from_square(Square::B5); 
    pub const C5: Self = Self::from_square(Square::C5); 
    pub const D5: Self = Self::from_square(Square::D5); 
    pub const E5: Self = Self::from_square(Square::E5); 
    pub const F5: Self = Self::from_square(Square::F5); 
    pub const G5: Self = Self::from_square(Square::G5); 
    pub const H5: Self = Self::from_square(Square::H5);
    
    pub const A6: Self = Self::from_square(Square::A6); 
    pub const B6: Self = Self::from_square(Square::B6); 
    pub const C6: Self = Self::from_square(Square::C6); 
    pub const D6: Self = Self::from_square(Square::D6); 
    pub const E6: Self = Self::from_square(Square::E6); 
    pub const F6: Self = Self::from_square(Square::F6); 
    pub const G6: Self = Self::from_square(Square::G6); 
    pub const H6: Self = Self::from_square(Square::H6);
    
    pub const A7: Self = Self::from_square(Square::A7); 
    pub const B7: Self = Self::from_square(Square::B7); 
    pub const C7: Self = Self::from_square(Square::C7); 
    pub const D7: Self = Self::from_square(Square::D7); 
    pub const E7: Self = Self::from_square(Square::E7); 
    pub const F7: Self = Self::from_square(Square::F7); 
    pub const G7: Self = Self::from_square(Square::G7); 
    pub const H7: Self = Self::from_square(Square::H7);
    
    pub const A8: Self = Self::from_square(Square::A8); 
    pub const B8: Self = Self::from_square(Square::B8); 
    pub const C8: Self = Self::from_square(Square::C8); 
    pub const D8: Self = Self::from_square(Square::D8); 
    pub const E8: Self = Self::from_square(Square::E8); 
    pub const F8: Self = Self::from_square(Square::F8); 
    pub const G8: Self = Self::from_square(Square::G8); 
    pub const H8: Self = Self::from_square(Square::H8);
}

impl std::convert::From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Self::from_square(value)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}
