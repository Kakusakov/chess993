use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Neg, Not, Shl, ShlAssign, Shr, ShrAssign 
};
use strum::EnumCount;
use crate::square::{
    File,
    Rank,
    PosDiag,
    NegDiag,
    Square,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitBoard(pub u64);

impl From<File> for BitBoard {
    fn from(value: File) -> Self {
        Self::from_file(value)
    }
}

impl From<Rank> for BitBoard {
    fn from(value: Rank) -> Self {
        Self::from_rank(value)
    }
}

impl From<PosDiag> for BitBoard {
    fn from(value: PosDiag) -> Self {
        Self::from_pos_diag(value)
    }
}

impl From<NegDiag> for BitBoard {
    fn from(value: NegDiag) -> Self {
        Self::from_neg_diag(value)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        Self::from_square(value)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::not(self)
    }
}

impl Neg for BitBoard {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::neg(self)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self::bitor(self, rhs)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::bitxor(self, rhs)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self::bitand(self, rhs)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl Shr<u8> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self::shr(self, rhs)
    }
}

impl ShrAssign<u8> for BitBoard {
    fn shr_assign(&mut self, rhs: u8) {
        *self = *self >> rhs
    }
}

impl Shl<u8> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self::shl(self, rhs)
    }
}

impl ShlAssign<u8> for BitBoard {
    fn shl_assign(&mut self, rhs: u8) {
        *self = *self << rhs
    }
}

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl BitBoard {
    pub const fn from_square(sq: Square) -> Self {
        Self(1).shl(sq as u8)
    }
    pub const fn from_rank(rank: Rank) -> Self {
        const RANK_ONE: BitBoard =
            BitBoard::from_square(Square::A1)
            .bitor(BitBoard::from_square(Square::B1))
            .bitor(BitBoard::from_square(Square::C1))
            .bitor(BitBoard::from_square(Square::D1))
            .bitor(BitBoard::from_square(Square::E1))
            .bitor(BitBoard::from_square(Square::F1))
            .bitor(BitBoard::from_square(Square::G1))
            .bitor(BitBoard::from_square(Square::H1));
        RANK_ONE.shl(File::COUNT as u8 * rank as u8)
    }
    pub const fn from_file(file: File) -> Self {
        const FILE_A: BitBoard =
            BitBoard::from_square(Square::A1)
            .bitor(BitBoard::from_square(Square::A2))
            .bitor(BitBoard::from_square(Square::A3))
            .bitor(BitBoard::from_square(Square::A4))
            .bitor(BitBoard::from_square(Square::A5))
            .bitor(BitBoard::from_square(Square::A6))
            .bitor(BitBoard::from_square(Square::A7))
            .bitor(BitBoard::from_square(Square::A8));
        FILE_A.shl(file as u8)
    }
    pub const fn from_pos_diag(diag: PosDiag) -> Self {
        const DIAG_A1H8: BitBoard =
            BitBoard::from_square(Square::A1)
            .bitor(BitBoard::from_square(Square::B2))
            .bitor(BitBoard::from_square(Square::C3))
            .bitor(BitBoard::from_square(Square::D4))
            .bitor(BitBoard::from_square(Square::E5))
            .bitor(BitBoard::from_square(Square::F6))
            .bitor(BitBoard::from_square(Square::G7))
            .bitor(BitBoard::from_square(Square::H8));
        DIAG_A1H8.genshift(diag as i8)
    }
    pub const fn from_neg_diag(diag: NegDiag) -> Self {
        const DIAG_A8H1: BitBoard =
            BitBoard::from_square(Square::A8)
            .bitor(BitBoard::from_square(Square::B7))
            .bitor(BitBoard::from_square(Square::C6))
            .bitor(BitBoard::from_square(Square::D5))
            .bitor(BitBoard::from_square(Square::E4))
            .bitor(BitBoard::from_square(Square::F3))
            .bitor(BitBoard::from_square(Square::G2))
            .bitor(BitBoard::from_square(Square::H1));
        DIAG_A8H1.genshift(diag as i8)
    }
    pub const fn up(self) -> Self {
        self.shl(File::COUNT as u8)
    }
    pub const fn down(self) -> Self {
        self.shr(File::COUNT as u8)
    }
    pub const fn right(self) -> Self {
        self.bitand(Self::FILE_H.not()).shl(1)
    }
    pub const fn left(self) -> Self {
        self.bitand(Self::FILE_A.not()).shr(1)
    }
    pub const fn not(self) -> Self {
        Self(!self.0)
    }
    pub const fn neg(self) -> Self {
        Self(self.0.wrapping_neg())
    }
    pub const fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
    pub const fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
    pub const fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
    pub const fn shr(self, rhs: u8) -> Self {
        Self(self.0 >> rhs)
    }
    pub const fn shl(self, rhs: u8) -> Self {
        Self(self.0 << rhs)
    }
    pub const fn genshift(self, rhs: i8) -> Self {
        if rhs >= 0 {
            self.shl(rhs as u8)
        } else {
            self.shr(-rhs as u8)
        }
    }
    pub const fn mul(self, rhs: Self) -> Self {
        Self(self.0.wrapping_mul(rhs.0))
    }
    pub const fn eq(self, rhs: Self) -> bool {
        self.0 == rhs.0
    }
    pub const fn is_empty(self) -> bool {
        self.eq(Self::EMPTY)
    }
    pub const fn fill_up(self) -> Self {
        self.mul(Self::FILE_A)
    }
    pub const fn attack_right(mut self, occupance: BitBoard) -> Self {
        let empty = occupance.not();
        self = self.right().bitand(empty);  // 1
        self = self.right().bitand(empty);  // 2
        self = self.right().bitand(empty);  // 3
        self = self.right().bitand(empty);  // 4
        self = self.right().bitand(empty);  // 5
        self = self.right().bitand(empty);  // 6
        self.right().bitand(empty)  // 7
    }
    pub const fn attack_left(mut self, occupance: BitBoard) -> Self {
        let empty = occupance.not();
        self = self.left().bitand(empty);  // 1
        self = self.left().bitand(empty);  // 2
        self = self.left().bitand(empty);  // 3
        self = self.left().bitand(empty);  // 4
        self = self.left().bitand(empty);  // 5
        self = self.left().bitand(empty);  // 6
        self.left().bitand(empty)  // 7
    }
    pub const fn attack_up(mut self, occupance: BitBoard) -> Self {
        let empty = occupance.not();
        self = self.up().bitand(empty);  // 1
        self = self.up().bitand(empty);  // 2
        self = self.up().bitand(empty);  // 3
        self = self.up().bitand(empty);  // 4
        self = self.up().bitand(empty);  // 5
        self = self.up().bitand(empty);  // 6
        self.up().bitand(empty)  // 7
    }
    pub const fn attack_down(mut self, occupance: BitBoard) -> Self {
        let empty = occupance.not();
        self = self.down().bitand(empty);  // 1
        self = self.down().bitand(empty);  // 2
        self = self.down().bitand(empty);  // 3
        self = self.down().bitand(empty);  // 4
        self = self.down().bitand(empty);  // 5
        self = self.down().bitand(empty);  // 6
        self.down().bitand(empty)  // 7
    }
    pub const fn rank_to_reversed_file(self) -> Self {
        self.mul(Self::DIAG_A1H8).shr(7).bitand(Self::FILE_A)
    }
    pub const fn file_to_reversed_rank(self) -> Self {
        self.mul(Self::DIAG_A1H8).shr(56)
    }
    pub const fn project_on_rank(self) -> Self {
        self.fill_up().shr(56)
    }
    // pub const fn mirror_horizontal(self) -> Self {
    //     self
    //     .mul(Self(0x80200802))
    //     .bitand(Self(0x0884422110))
    //     .mul(Self(0x0101010101010101))
    //     .shl(56)
    // }
}

impl BitBoard {
    pub const EMPTY: Self = Self(0);

    pub const FILE_A: Self = Self::from_file(File::A);
    pub const FILE_B: Self = Self::from_file(File::B);
    pub const FILE_C: Self = Self::from_file(File::C);
    pub const FILE_D: Self = Self::from_file(File::D);
    pub const FILE_E: Self = Self::from_file(File::E);
    pub const FILE_F: Self = Self::from_file(File::F);
    pub const FILE_G: Self = Self::from_file(File::G);
    pub const FILE_H: Self = Self::from_file(File::H);

    pub const RANK_ONE: Self = Self::from_rank(Rank::One);
    pub const RANK_TWO: Self = Self::from_rank(Rank::Two);
    pub const RANK_THREE: Self = Self::from_rank(Rank::Three);
    pub const RANK_FOUR: Self = Self::from_rank(Rank::Four);
    pub const RANK_FIVE: Self = Self::from_rank(Rank::Five);
    pub const RANK_SIX: Self = Self::from_rank(Rank::Six);
    pub const RANK_SEVEN: Self = Self::from_rank(Rank::Seven);
    pub const RANK_EIGHT: Self = Self::from_rank(Rank::Eight);

    pub const DIAG_H1H1: Self = Self::from_pos_diag(PosDiag::H1H1);
    pub const DIAG_G1H2: Self = Self::from_pos_diag(PosDiag::G1H2);
    pub const DIAG_F1H3: Self = Self::from_pos_diag(PosDiag::F1H3);
    pub const DIAG_E1H4: Self = Self::from_pos_diag(PosDiag::E1H4);
    pub const DIAG_D1H5: Self = Self::from_pos_diag(PosDiag::D1H5);
    pub const DIAG_C1H6: Self = Self::from_pos_diag(PosDiag::C1H6);
    pub const DIAG_B1H7: Self = Self::from_pos_diag(PosDiag::B1H7);
    pub const DIAG_A1H8: Self = Self::from_pos_diag(PosDiag::A1H8);
    pub const DIAG_A2G8: Self = Self::from_pos_diag(PosDiag::A2G8);
    pub const DIAG_A3F8: Self = Self::from_pos_diag(PosDiag::A3F8);
    pub const DIAG_A4E8: Self = Self::from_pos_diag(PosDiag::A4E8);
    pub const DIAG_A5D8: Self = Self::from_pos_diag(PosDiag::A5D8);
    pub const DIAG_A6C8: Self = Self::from_pos_diag(PosDiag::A6C8);
    pub const DIAG_A7B8: Self = Self::from_pos_diag(PosDiag::A7B8);
    pub const DIAG_A8A8: Self = Self::from_pos_diag(PosDiag::A8A8);

    pub const DIAG_A1A1: Self = Self::from_neg_diag(NegDiag::A1A1);
    pub const DIAG_A2B1: Self = Self::from_neg_diag(NegDiag::A2B1);
    pub const DIAG_A3C1: Self = Self::from_neg_diag(NegDiag::A3C1);
    pub const DIAG_A4D1: Self = Self::from_neg_diag(NegDiag::A4D1);
    pub const DIAG_A5E1: Self = Self::from_neg_diag(NegDiag::A5E1);
    pub const DIAG_A6F1: Self = Self::from_neg_diag(NegDiag::A6F1);
    pub const DIAG_A7G1: Self = Self::from_neg_diag(NegDiag::A7G1);
    pub const DIAG_A8H1: Self = Self::from_neg_diag(NegDiag::A8H1);
    pub const DIAG_B8H2: Self = Self::from_neg_diag(NegDiag::B8H2);
    pub const DIAG_C8H3: Self = Self::from_neg_diag(NegDiag::C8H3);
    pub const DIAG_D8H4: Self = Self::from_neg_diag(NegDiag::D8H4);
    pub const DIAG_E8H5: Self = Self::from_neg_diag(NegDiag::E8H5);
    pub const DIAG_F8H6: Self = Self::from_neg_diag(NegDiag::F8H6);
    pub const DIAG_G8H7: Self = Self::from_neg_diag(NegDiag::G8H7);
    pub const DIAG_H8H8: Self = Self::from_neg_diag(NegDiag::H8H8);

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
