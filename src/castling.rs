use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Castling: u8 {
        const WHITE = 0b00_11;
        const BLACK = 0b11_00;
        const KING = 0b01_01;
        const QUEEN = 0b10_10;
    }
}
