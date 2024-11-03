use std::ops::BitAnd;

use crate::color::Color;
use crate::bitboard::BitBoard;
use crate::square::{File, Rank, Square};

#[derive(Debug)]
pub struct Movegen;

impl Movegen {
    pub const fn pawn_quiet(from: Square, color: Color) -> BitBoard {
        let bb = BitBoard::from_square(from);
        match color {
            Color::White => bb.up(),
            Color::Black => bb.down(),
        }
    }
    pub const fn pawn_attack(from: Square, color: Color) -> BitBoard {
        let bb = Self::pawn_quiet(from, color);
        bb.left().bitor(bb.right())
    }
    pub const fn knight_move(from: Square) -> BitBoard {
        const MAGIC_L: BitBoard = BitBoard::EMPTY.not().left();
        const MAGIC_LL: BitBoard = MAGIC_L.left();
        const MAGIC_R: BitBoard = BitBoard::EMPTY.not().right();
        const MAGIC_RR: BitBoard = MAGIC_R.right();
        let bb = BitBoard::from_square(from);
        let l1 = bb.shr(1).bitand(MAGIC_L);
        let l2 = bb.shr(2).bitand(MAGIC_LL);
        let r1 = bb.shl(1).bitand(MAGIC_R);
        let r2 = bb.shl(2).bitand(MAGIC_RR);
        let h1 = l1.bitor(r1);
        let h2 = l2.bitor(r2);
        h1.shl(16)
        .bitor(h1.shr(16))
        .bitor(h2.shl(8))
        .bitor(h2.shr(8))
    }
    pub const fn king_move(from: Square) -> BitBoard {
        let bb = BitBoard::from_square(from);
        let tmp = bb.left().bitor(bb.right());
        tmp
        .bitor(tmp.up())
        .bitor(tmp.down())
        .bitor(bb.up())
        .bitor(bb.down())
    }
    pub const fn diagonal_move(from: Square, occupance: BitBoard) -> BitBoard {
        Self::pos_diag_attacks(from, occupance)
        .bitor(Self::neg_diag_attacks(from, occupance))
    }
    pub const fn straight_move(from: Square, occupance: BitBoard) -> BitBoard {
        Self::rank_attacks(from, occupance)
        .bitor(Self::file_attack(from, occupance))
    }
}

impl Movegen {
    const fn pos_diag_attacks(from: Square, occ: BitBoard) -> BitBoard {
        let mask = BitBoard::from_pos_diag(from.pos_diag());
        let occ_6bit = Self::inner_6bits(mask.bitand(occ).project_on_rank());
        mask.bitand(Self::fill_up_attack(from.file(), occ_6bit))
    }
    const fn neg_diag_attacks(from: Square, occ: BitBoard) -> BitBoard {
        let mask = BitBoard::from_neg_diag(from.neg_diag());
        let occ_6bit = Self::inner_6bits(mask.bitand(occ).project_on_rank());
        mask.bitand(Self::fill_up_attack(from.file(), occ_6bit))
    }
    const fn rank_attacks(from: Square, occ: BitBoard) -> BitBoard {
        let mask = BitBoard::from_rank(from.rank());
        let occ_6bit = Self::inner_6bits(mask.bitand(occ).project_on_rank());
        mask.bitand(Self::fill_up_attack(from.file(), occ_6bit))
    }
    const fn file_attack(from: Square, occupance: BitBoard) -> BitBoard {
        let rank = from.rank();
        let file = from.file();
        let file_occ = BitBoard::from_file(File::A).bitand(occupance.shr(file as u8));
        let rev_occ = file_occ.file_to_reversed_rank();
        let rev_occ_6bit = Self::inner_6bits(rev_occ);
        Self::file_a_attack(rank, rev_occ_6bit).shl(file as u8)
    }
    const fn fill_up_attack(file: File, occ_6bit: u8) -> BitBoard {
        assert!(occ_6bit < 64);
        let occ = BitBoard(occ_6bit as u64).shl(1);
        let slider = BitBoard::from_square(Square::straights(Rank::One, file));
        slider.attack_left(occ).bitor(slider.attack_right(occ))
    }
    const fn file_a_attack(rank: Rank, rev_occ_6bit: u8) -> BitBoard {
        assert!(rev_occ_6bit < 64);
        let rev_occ = BitBoard(rev_occ_6bit as u64).shl(1);
        let occ = rev_occ.rank_to_reversed_file();
        let slider = BitBoard::from_square(Square::straights(rank, File::A));
        slider.attack_up(occ).bitor(slider.attack_down(occ))
    }
    const fn inner_6bits(bb_rank: BitBoard) -> u8 {
        assert!(bb_rank.bitand(BitBoard::from_rank(Rank::One).not()).is_empty());
        bb_rank.bitand(BitBoard::from_square(Square::H1).not()).shr(1).0 as u8
    }
}
