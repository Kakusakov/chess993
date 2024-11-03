use std::ops::{
    Not,
    Neg,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Shl, ShlAssign,
    Shr, ShrAssign,
    Mul,
};
use strum::EnumCount;
use crate::square::{
    File,
    Rank,
    PosDiag,
    NegDiag,
    Square,
};

#[derive(Debug, Clone, Copy, Default)]
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

impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        Self::eq(self, other)
    }
}
impl Eq for BitBoard {}

impl BitBoard {
    pub const EMPTY: Self = Self(u64::MIN);
    pub const FULL: Self = Self(u64::MAX);
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
        self.bitand(Self::from_file(File::H).not()).shl(1)
    }
    pub const fn left(self) -> Self {
        self.bitand(Self::from_file(File::A).not()).shr(1)
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
    pub const fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
    pub const fn ne(&self, rhs: &Self) -> bool {
        !self.eq(rhs)
    }
    pub const fn is_empty(self) -> bool {
        self.eq(&Self::EMPTY)
    }
    pub const fn is_full(self) -> bool {
        self.eq(&Self::FULL)
    }
    pub const fn has_square(self, sq: Square) -> bool {
        !self.bitand(Self::from_square(sq)).is_empty()
    }
    pub const fn fill_up(self) -> Self {
        self.mul(Self::from_file(File::A))
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
        self.mul(Self::from_pos_diag(PosDiag::A1H8)).shr(7).bitand(Self::from_file(File::A))
    }
    pub const fn file_to_reversed_rank(self) -> Self {
        self.mul(Self::from_pos_diag(PosDiag::A1H8)).shr(56)
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
