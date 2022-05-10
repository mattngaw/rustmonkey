//! Represents a chess board and provides an interface for changing the state
//! of the board.
//!
//!

pub mod bits;
mod castling;
mod square_lut;
mod util;

use crate::movegen::Move;
use crate::helper::{king_origin, rook_origin};
use bits::{File, Rank, Square, Bitboard};
use castling::Castling;
use square_lut::SquareLUT;

/// The six piece types in chess
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    Null,
    Empty,
    Pc(Whose, PieceType),
}

impl Piece {
    // Coverts a piece to its
    pub fn to_char(&self) -> char {
        match self {
            Piece::Null => panic!("Attempted to convert Piece::Null to char"),
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

    pub fn is_whose(&self, w: Whose) -> bool {
        match *self {
            Piece::Pc(w_, _) => (w == w_),
            Piece::Empty => false,
            Piece::Null => panic!("Attempted to check Whose of null piece")
        }
    }

    pub fn is_piecetype(&self, pt: PieceType) -> bool {
        match *self {
            Piece::Pc(_, pt_) => (pt == pt_),
            Piece::Empty => false,
            Piece::Null => panic!("Attempted to check PieceType of null piece")
        }
    }
}

/// Denotes whose turn it is
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Flips a Color
    pub fn flip(&mut self) -> () {
        match *self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
        }
    }
}

/// Represents the two sides where one can castle
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Side { K, Q }

/// All the components combined to represent a chess board
///
/// 
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Board {
    pub whose_bbs: [Bitboard; Whose::COUNT],
    pub piece_type_bbs: [Bitboard; PieceType::NK_COUNT],
    pub kings: [Square; Whose::COUNT],
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
            whose_bbs: [Bitboard::Null; Whose::COUNT], 
            piece_type_bbs: [Bitboard::Null; PieceType::NK_COUNT], 
            kings: [Square::Null; Whose::COUNT], 
            sq_lut: SquareLUT::new(), 
            castling: Castling::EMPTY, 
            color: Color::White, 
            en_passant: Square::Null, 
            half_moves: 0u8, 
            rule50: 0u8, 
        }
    }

    pub fn clear(&mut self) -> () {
        for wbb in &mut self.whose_bbs { *wbb = Bitboard::EMPTY; }
        for ptbb in &mut self.piece_type_bbs { *ptbb = Bitboard::EMPTY; }
        self.sq_lut.clear();
    }

    /// Gets the piece at a [Square](crate::board::bits::Square)
    pub fn get (&self, sq: Square) -> Piece {
        match sq {
            Square::Null => panic!("Attempted to get from Board at Square::Null"),
            Square::Sq(_) => self.sq_lut.get(sq)
        }
    }

    /// Sets the piece at a [Square](crate::board::bits::Square)
    /// 
    /// Removes the piece (if there is one) that was originally on `sq`,
    /// and then replaces it with the new piece `p`
    pub fn set (&mut self, sq: Square, p: Piece) -> () {
        match sq {
            Square::Null => panic!("Attempted to set on Board at Square::Null"),
            Square::Sq(_) => {
                let p_prev = self.get(sq);
                match p_prev {
                    Piece::Null => (),
                    Piece::Empty => (),
                    Piece::Pc(w_prev, pt_prev) => {
                        self.whose_bbs[w_prev as usize].reset(sq);
                        if pt_prev == PieceType::K {
                            self.kings[w_prev as usize] = Square::Null;
                        } else {
                            self.piece_type_bbs[pt_prev as usize].reset(sq);
                        }
                    }
                }
                match p {
                    Piece::Null => panic!("Attempted to set Piece::Null on Board"),
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

    /// Returns a bitboard representing all of the squares with pieces on them
    pub fn get_all(&self) -> Bitboard {
        self.whose_bbs[Whose::Ours as usize] | self.whose_bbs[Whose::Theirs as usize]
    }

    /// Returns a bitboard representing all of the squares with `w: Whose` 
    /// pieces on them
    pub fn get_whose(&self, w: Whose) -> Bitboard {
        self.whose_bbs[w as usize]
    }

    /// Returns a bitboard representing all of the squares with `p: Piece` 
    /// pieces on them
    pub fn get_pieces(&self, p: Piece) -> Bitboard {
        match p {
            Piece::Pc(w, pt) => {
                self.whose_bbs[w as usize] & self.piece_type_bbs[pt as usize]
            }
            Piece::Empty => panic!("Attempted to get empty piece"),
            Piece::Null => panic!("Attempted to get null piece")
        }
    }

    /// Moves a [Piece] from a square to another square
    pub fn move_piece(&mut self, to: Square, from: Square) -> () {
        let p_from = self.get(from);
        match p_from {
            Piece::Null => panic!("Attempted to move Piece::Null"),
            Piece::Empty => panic!("Attempted to move Piece::Empty"),
            Piece::Pc(_, _) => self.set(from, Piece::Empty),
        };
        let p_to = self.get(to);
        match p_to {
            Piece::Null => panic!("Attempted to move to a Piece::Null"),
            _ => self.set(to, p_from),
        }
    }

    /// Updates the board given a valid move
    pub fn apply(&mut self, m: Move) -> () {
        if m.castling.is_some() {
            let cs = m.castling.unwrap();
            self.apply_castling(cs);
        } else if m.promotion.is_some() {
            self.set(m.from, Piece::Empty);
            let promo_pt = m.promotion.unwrap();
            self.set(m.to, Piece::Pc(Whose::Ours, promo_pt));
        } else {
            let moved_piece = self.get(m.from);
            self.move_piece(m.to, m.from);
            match moved_piece {
                Piece::Pc(Whose::Ours, PieceType::K) => 
                    self.castling_reset_both(Whose::Ours),
                Piece::Pc(Whose::Ours, PieceType::R) => {
                    for side in [Side::K, Side::Q] {
                        let origin = rook_origin(side, Whose::Ours, self.color);
                        let castling = self.castling_get(Whose::Ours, side);
                        if castling && m.from == origin {
                            self.castling_reset(Whose::Ours, side);
                        }
                    }
                }
                _ => ()
            }
            if m.dpp {
                debug_assert_eq!(moved_piece, Piece::Pc(Whose::Ours, PieceType::P));
                let left = m.to.file_down();
                let right = m.to.file_up();
                let their_pawn = Piece::Pc(Whose::Theirs, PieceType::P);
                for sq in [left, right] {
                    match sq {
                        Square::Sq(_) => {
                            if self.get(sq) == their_pawn {
                                self.en_passant = m.to.rank_down();
                            } else {
                                self.en_passant = Square::Null;
                            }
                        }
                        Square::Null => ()
                    }
                }
            }
        }
    }

    /// Helper function, applies castling to the board
    fn apply_castling (&mut self, cs: Side) -> () {
        debug_assert!(self.castling_get(Whose::Ours, cs));
        let king = match self.color {
            Color::White => Square::from(File::E, Rank::First),
            Color::Black => Square::from(File::E, Rank::Eighth)
        };
        let rook = match (self.color, cs) {
            (Color::White, Side::K) => Square::from(File::H, Rank::First),
            (Color::White, Side::Q) => Square::from(File::A, Rank::First),
            (Color::Black, Side::K) => Square::from(File::H, Rank::Eighth),
            (Color::Black, Side::Q) => Square::from(File::A, Rank::Eighth),
        };
        let (king_new, rook_new) = match cs {
            Side::K => (king.offset(2i8, 0), rook.offset(-2i8, 0)),
            Side::Q => (king.offset(-2i8, 0), rook.offset(3i8, 0)),
        };
        self.move_piece(king_new, king);
        self.move_piece(rook_new, rook);
        self.castling_reset_both(Whose::Ours);
    }

    /// Flips an entire board
    pub fn flip (&mut self) -> () {
        for wbb in &mut self.whose_bbs { wbb.flip(); }
        for ptbb in &mut self.piece_type_bbs { ptbb.flip(); }
        for k in &mut self.kings { k.flip(); }
        self.sq_lut.flip();
        self.castling.flip();
        self.color.flip();
    }

    /// Gets castling rights
    pub fn castling_get(&mut self, w: Whose, cs: Side) -> bool {
        self.castling.get(w, cs)
    }

    /// Sets castling rights
    pub fn castling_set(&mut self, w: Whose, cs: Side) -> () {
        self.castling.set(w, cs)
    }

    /// Resets castling rights
    pub fn castling_reset(&mut self, w: Whose, cs: Side) -> () {
        self.castling.reset(w, cs)
    }

    /// Resets castling rights
    pub fn castling_reset_both(&mut self, w: Whose) -> () {
        self.castling.reset(w, Side::K);
        self.castling.reset(w, Side::Q);
    }

    /// Prints the board
    /// 
    /// Our pieces are uppercase, theirs are lowercase
    pub fn print (&self) -> () {
        self.sq_lut.print();
        println!("");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    fn test_board_get_set() {
        let mut bd = Board::new();
        bd.whose_bbs = [Bitboard::EMPTY; Whose::COUNT];
        bd.piece_type_bbs = [Bitboard::EMPTY; PieceType::NK_COUNT];
        assert_eq!(bd.get(Square::Sq(16)), Piece::Null);
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

    // #[test]
    fn test_apply() {
        let mut bd = Board::new();
        bd.clear();
        bd.set(Square::from(File::E, Rank::Second), 
               Piece::Pc(Whose::Ours, PieceType::P));
        bd.print();
        let m = Move {
            to: Square::from(File::E, Rank::Fourth),
            from: Square::from(File::E, Rank::Second),
            capture: false,
            dpp: true,
            promotion: None,
            castling: None
        };
        bd.apply(m);
        bd.print();
    }
}
