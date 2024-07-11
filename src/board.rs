macro_rules! from_u8 {
    ($t:ty) => {
        impl $t {
            pub const fn from_u8(value: u8) -> Self {
                assert!((value as usize) < enum_iterator::cardinality::<Self>());
                unsafe {
                    std::mem::transmute(value)
                }
            }
        }
    };
}

mod color;
mod square;
mod piece;
mod bitboard;
mod castling_rights;

pub use color::*;
pub use square::*;
pub use piece::*;
pub use bitboard::*;
pub use castling_rights::*;

struct Board {}

impl Board {}
