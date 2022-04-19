//! Represents a chess board and provides an interface for changing the state
//! of the board.
//!
//!

pub mod bits;
mod castling;
mod square_lut;
mod util;

use bits::{Square, Bitboard};
use castling::{CastlingSide, Castling};
use square_lut::{SquareLUT};

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
    /// Number of piece types in chess
    pub const COUNT: usize = 6usize;

    /// Number of piece types in chess minus the kings
    /// 
    /// Used when iterating over piece bitboards 
    /// (the king doesn't get a bitboard)
    pub const NK_COUNT: usize = 5usize;
}


/// Enum for colorless piece representation
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Whose {
    Ours,
    Theirs,
}

impl Whose {
    // Number of sides to the game (two sides, duh)
    pub const COUNT: usize = 2usize;

    pub fn flip (&mut self) -> () {
        match *self {
            Whose::Ours => *self = Whose::Theirs,
            Whose::Theirs => *self = Whose::Ours,
        }
    }
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
    // Coverts a piece to its
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn flip(&mut self) -> () {
        match *self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
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
    kings: [Square; Whose::COUNT],
    sq_lut: SquareLUT,
    castling: Castling,
    color: Color,
    en_passant: Square,
    half_moves: u8,
    rule50: u8,
}

impl Board {
    /// Creates a new board
    /// 
    /// Caveat emptor: most fields are initialized to null values and must be 
    /// set before use
    pub fn new() -> Board {
        Board { 
            whose_bbs: [Bitboard::NullBb; Whose::COUNT], 
            piece_type_bbs: [Bitboard::NullBb; PieceType::NK_COUNT], 
            kings: [Square::NullSq; Whose::COUNT], 
            sq_lut: SquareLUT::new(), 
            castling: Castling::EMPTY, 
            color: Color::White, 
            en_passant: Square::NullSq, 
            half_moves: 0u8, 
            rule50: 0u8, 
        }
    }

    pub fn get (&self, sq: Square) -> Piece {
        match sq {
            Square::NullSq => panic!("Attempted to get from Board at NullSq"),
            Square::Sq(_) => self.sq_lut.get(sq)
        }
    }

    pub fn set (&mut self, sq: Square, p: Piece) -> () {
        match sq {
            Square::NullSq => panic!("Attempted to set on Board at NullSq"),
            Square::Sq(_) => {
                let p_prev = self.get(sq);
                match p_prev {
                    Piece::NullPc => (),
                    Piece::Empty => (),
                    Piece::Pc(w_prev, pt_prev) => {
                        self.whose_bbs[w_prev as usize].reset(sq);
                        if pt_prev == PieceType::K {
                            self.kings[w_prev as usize] = Square::NullSq;
                        } else {
                            self.piece_type_bbs[pt_prev as usize].reset(sq);
                        }
                    }
                }
                match p {
                    Piece::NullPc => panic!("Attempted to set NullPc on Board"),
                    Piece::Empty => (),
                    Piece::Pc(w, pt) => {
                        self.whose_bbs[w as usize].set(sq);
                        if pt == PieceType::K {
                            self.kings[w as usize] = sq;
                        } else {
                            self.piece_type_bbs[pt as usize].set(sq);
                        }
                    }
                }
                self.sq_lut.set(sq, p);
            }
        }
    }

    pub fn move_piece (&mut self, to: Square, from: Square) -> () {
        let p_from = self.get(from);
        match p_from {
            Piece::NullPc => panic!("Attempted to move NullPc"),
            Piece::Empty => panic!("Attempted to move Empty"),
            Piece::Pc(_, _) => self.set(from, Piece::Empty),
        };
        let p_to = self.get(to);
        match p_to {
            Piece::NullPc => panic!("Attempted to move to a NullPc"),
            _ => self.set(to, p_from),
        }
    }

    pub fn flip (&mut self) -> () {
        for wbb in &mut self.whose_bbs { wbb.flip(); }
        for ptbb in &mut self.piece_type_bbs { ptbb.flip(); }
        for k in &mut self.kings { k.flip(); }
        self.sq_lut.flip();
        self.castling.flip();
        self.color.flip();
    }

    pub fn print (&self) -> () {
        self.sq_lut.print();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_board_get_set() {
        let mut bd = Board::new();
        bd.whose_bbs = [Bitboard::EMPTY; Whose::COUNT];
        bd.piece_type_bbs = [Bitboard::EMPTY; PieceType::NK_COUNT];
        assert_eq!(bd.get(Square::Sq(16)), Piece::NullPc);
        for i in 0u8..64u8 {
            bd.set(Square::new(i), Piece::Empty);
        }
        bd.set(Square::new(0u8), Piece::Pc(Whose::Ours, PieceType::K));
        bd.set(Square::new(60u8), Piece::Pc(Whose::Theirs, PieceType::K));
        bd.set(Square::Sq(16), Piece::Pc(Whose::Ours, PieceType::N));
        assert_eq!(bd.get(Square::Sq(16)), Piece::Pc(Whose::Ours, PieceType::N));
        assert_eq!(bd.get(Square::Sq(0)), Piece::Pc(Whose::Ours, PieceType::K));
        assert_eq!(bd.get(Square::Sq(60)), Piece::Pc(Whose::Theirs, PieceType::K));
        bd.flip();
        assert_eq!(bd.get(Square::Sq(47)), Piece::Pc(Whose::Theirs, PieceType::N));
        assert_eq!(bd.get(Square::Sq(63)), Piece::Pc(Whose::Theirs, PieceType::K));
        assert_eq!(bd.get(Square::Sq(3)), Piece::Pc(Whose::Ours, PieceType::K));
    }
}