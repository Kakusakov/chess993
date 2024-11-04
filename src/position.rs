use bytemuck::Contiguous;
use strum::EnumCount;

use crate::bitboard::BitBoard;
use crate::bitboards::{Bitboards};
use crate::en_passant::{EnPassant};
use crate::ply_clock::{PlyClock};
use crate::piece::{CapturablePiece, Piece, PromotablePiece};
use crate::square::{File, Rank, Square};
use crate::color::Color;
use crate::castling::{Castling, CastlingSide};
use crate::movegen::Movegen;

struct Position {
    bitboards: Bitboards,
    en_passant: EnPassant,
    ply_clock: PlyClock,
    castling: Castling,
    turn: Color,
}

enum CompactChessMove {
    Quiet {
        from: Square,
        to: Square,
    },
    Capture {
        from: Square,
        to: Square,
    },
    PawnPush {
        from: Square,
        to: Square,
    },
    DoublePawnPush {
        from: Square,
        to: Square,
    },
    EnPassant {
        from: Square,
        to: Square,
    },
    Castling(Castling),
    QuietPromotion {
        from: Square,
        // to is deduced.
        promotion: PromotablePiece,
    },
    CaptureProtion {
        from: Square,
        to: Square,
        promotion: PromotablePiece,
    },
}

// #[derive(Debug, Clone, Copy)]
// pub struct Position {
//     bitboards: BitBoards,
//     state: State,
// }

// #[derive(Debug, Clone, Copy)]
// pub struct State {
//     turn: Color,
//     castling_rights: CastlingRights,
//     en_passant: Option<File>,
//     halfmove_clock: u8,
// }

// impl State {
//     pub fn next(&mut self) {
//         self.turn = !self.turn;
//         self.en_passant = None;
//         self.halfmove_clock = self.halfmove_clock.saturating_add(1);
//     }
//     pub fn reset_clock(&mut self) {
//         self.halfmove_clock = 0;
//     }
//     pub fn set_en_passant(&mut self, file: File) {
//         self.en_passant = Some(file);
//     }
//     pub fn piece_was_updated(&mut self, color: Color, piece: Piece, sq: Square) {
//         self.castling_rights &= !match piece {
//             Piece::King => CastlingRights::from_color(color),
//             Piece::Rook => match (color, sq) {
//                 (Color::White, Square::A1) => CastlingRights::WHITE_QUEEN,
//                 (Color::White, Square::A8) => CastlingRights::WHITE_KING,
//                 (Color::Black, Square::H1) => CastlingRights::BLACK_QUEEN,
//                 (Color::Black, Square::H8) => CastlingRights::BLACK_KING,
//                 _ => CastlingRights::NONE,
//             }
//             _ => CastlingRights::NONE,
//         }
//     }
// }

// impl Position {
//     pub fn is_move_valid(&self, chess_move: ChessMove) -> bool {
//         todo!()
//     }
//     pub fn is_unmove_valid(&self, chess_unmove: ChessUnmove) -> bool {
//         todo!()
//     }
//     pub fn make_move(&mut self, chess_move: ChessMove) -> ChessUnmove {
//         assert!(self.is_move_valid(chess_move));
//         let state = self.state;
//         self.state.next();
//         let raw_chess_unmove = match chess_move.0 {
//             RawChessMove::Quiet {
//                 from,
//                 to
//             } => {
//                 let piece = self.bitboards.piece_at(from).unwrap();
//                 let color = self.state.turn;
//                 self.state.piece_was_updated(color, piece, from);
//                 self.bitboards.move_piece(color, piece, from, to);
//                 RawChessUnmove::Quiet {
//                     from,
//                     to,
//                 }
//             },
//             RawChessMove::Capture {
//                 from,
//                 to
//             } => {
//                 let piece = self.bitboards.piece_at(from).unwrap();
//                 let capture = CapturablePiece::from_piece(
//                     self.bitboards.piece_at(to).unwrap()
//                 ).unwrap();
//                 let color = self.state.turn;
//                 self.state.piece_was_updated(color, piece, from);
//                 self.state.piece_was_updated(!color, capture.into(), to);
//                 self.state.reset_clock();
//                 self.bitboards.capture_piece(!color, capture, to);
//                 self.bitboards.move_piece(color, piece, from, to);
//                 RawChessUnmove::Capture {
//                     from,
//                     to,
//                     capture,
//                 }
//             },
//             RawChessMove::PawnPush {
//                 from
//             } => {
//                 let color = self.state.turn;
//                 let to = Square::straights(
//                     match color {
//                         Color::White => Rank::Three,
//                         Color::Black => Rank::Six,
//                     },
//                     from.file()
//                 );
//                 self.state.reset_clock();
//                 self.bitboards.move_piece(color, Piece::Pawn, from, to);
//                 RawChessUnmove::PawnPush {
//                     from,
//                     to,
//                 }
//             },
//             RawChessMove::DoublePawnPush {
//                 from
//             } => {
//                 let color = self.state.turn;
//                 let to = Square::straights(
//                     match color {
//                         Color::White => Rank::Four,
//                         Color::Black => Rank::Five,
//                     },
//                     from.file()
//                 );
//                 self.state.reset_clock();
//                 self.state.set_en_passant(from.file());
//                 self.bitboards.move_piece(color, Piece::Pawn, from, to);
//                 RawChessUnmove::DoublePawnPush {
//                     from,
//                     to,
//                 }
//             },
//             RawChessMove::EnPassant {
//                 from,
//                 to
//             } => {
//                 let capture_sq = Square::straights(
//                     from.rank(),
//                     to.file(),
//                 );
//                 let color = self.state.turn;
//                 self.state.reset_clock();
//                 self.bitboards.capture_piece(!color, CapturablePiece::Pawn, capture_sq);
//                 self.bitboards.move_piece(color, Piece::Pawn, from, to);
//                 RawChessUnmove::EnPassant {
//                     from,
//                     to,
//                 }
//             },
//             RawChessMove::QuietPromotion {
//                 from,
//                 promotion
//             } => {
//                 let color = self.state.turn;
//                 let to = Square::straights(
//                     match color {
//                         Color::White => Rank::Eight,
//                         Color::Black => Rank::One,
//                     }, 
//                     from.file()
//                 );
//                 self.state.reset_clock();
//                 self.bitboards.promote_piece(color, promotion, from, to);
//                 RawChessUnmove::QuietPromotion {
//                     from,
//                     to,
//                     promotion
//                 }
//             },
//             RawChessMove::CapturePromotion {
//                 from,
//                 to,
//                 promotion
//             } => {
//                 let color = self.state.turn;
//                 self.state.reset_clock();
//                 let capture = CapturablePiece::from_piece(
//                     self.bitboards.piece_at(to).unwrap()
//                 ).unwrap();
//                 self.state.piece_was_updated(!color, capture.into(), to);
//                 self.bitboards.capture_piece(!color, capture, to);
//                 self.bitboards.promote_piece(color, promotion, from, to);
//                 RawChessUnmove::CapturePromotion {
//                     from,
//                     to,
//                     promotion,
//                     capture,
//                 }
//             },
//             RawChessMove::Castling(castling) => todo!(),
//         };
//         ChessUnmove(raw_chess_unmove, state)
//     }
//     pub fn make_unmove(&mut self, chess_unmove: ChessUnmove) {
//         assert!(self.is_unmove_valid(chess_unmove));
//         todo!()
//     }
// }
