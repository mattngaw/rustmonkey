//! Represents a chess board and provides an interface for changing the state
//! of the board.
//!
//!

pub mod bits;
pub mod castling;
pub mod square_lut;
pub mod util;

use bits::*;
use castling::*;

/// The six piece types in chess
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    P,
    N,
    B,
    R,
    Q,
    K,
}

impl PieceType {
    const COUNT: usize = 6usize;
    const NK_COUNT: usize = 5usize;
}


/// Enum for colorless piece representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Whose {
    Ours,
    Theirs,
}

impl Whose {
    const COUNT: usize = 2usize;
}

/// Tuple of [`PieceType`] and [`Whose`]
///
/// Used in [`SquareLUT`](square_lut::SquareLUT)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    NullPc,
    Empty,
    Pc(Whose, PieceType),
}

impl Piece {
    pub fn to_char(&self) -> char {
        match self {
            Piece::NullPc => panic!("Attempted to convert NullPc to char"),
            Piece::Empty => '.',
            Piece::Pc(Whose::Ours, PieceType::P) => 'P',
            Piece::Pc(Whose::Ours, PieceType::N) => 'N',
            Piece::Pc(Whose::Ours, PieceType::B) => 'B',
            Piece::Pc(Whose::Ours, PieceType::R) => 'R',
            Piece::Pc(Whose::Ours, PieceType::Q) => 'Q',
            Piece::Pc(Whose::Ours, PieceType::K) => 'K',
            Piece::Pc(Whose::Theirs, PieceType::P) => 'p',
            Piece::Pc(Whose::Theirs, PieceType::N) => 'n',
            Piece::Pc(Whose::Theirs, PieceType::B) => 'b',
            Piece::Pc(Whose::Theirs, PieceType::R) => 'r',
            Piece::Pc(Whose::Theirs, PieceType::Q) => 'q',
            Piece::Pc(Whose::Theirs, PieceType::K) => 'k',
        }
    }
}

/// All the components combined to represent a chess board
///
///
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Board {
    whose_bbs: [Bitboard; Whose::COUNT],
    piece_type_bbs: [Bitboard; PieceType::NK_COUNT],
    K: [Square; Whose::COUNT],
    castling: Castling,
    whose: Whose,
    en_passant: Square,
}
