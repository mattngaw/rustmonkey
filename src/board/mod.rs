//! Represents a chess board and provides an interface for changing the state
//! of the board.
//!
//!

pub mod bits;
mod castling;
mod square_lut;
mod util;

use bits::{Square, Bitboard, Rank, File};
use castling::{Side, Castling};
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

    pub fn get (&self, sq: Square) -> Piece {
        match sq {
            Square::Null => panic!("Attempted to get from Board at Square::Null"),
            Square::Sq(_) => self.sq_lut.get(sq)
        }
    }

    pub fn set (&mut self, sq: Square, p: Piece) -> () {
        // print!("hello\n");
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

    pub fn move_piece (&mut self, to: Square, from: Square) -> () {
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

    pub fn flip (&mut self) -> () {
        for wbb in &mut self.whose_bbs { wbb.flip(); }
        for ptbb in &mut self.piece_type_bbs { ptbb.flip(); }
        for k in &mut self.kings { k.flip(); }
        self.sq_lut.flip();
        self.castling.flip();
        self.color.flip();
    }

    pub fn castling_get(&mut self, w: Whose, cs: Side) -> bool {
        self.castling.get(w, cs)
    }

    pub fn castling_set(&mut self, w: Whose, cs: Side) -> () {
        self.castling.set(w, cs)
    }

    pub fn castling_reset(&mut self, w: Whose, cs: Side) -> () {
        self.castling.reset(w, cs)
    }

    pub fn print (&self) -> () {
        self.sq_lut.print();
    }

    pub fn board_clear (&mut self) -> () {
        self.whose_bbs = [Bitboard::EMPTY; Whose::COUNT];
        self.piece_type_bbs = [Bitboard::EMPTY; PieceType::NK_COUNT];
        self.kings = [Square::Null; Whose::COUNT];
        self.sq_lut = SquareLUT::new();
        self.castling = Castling::EMPTY; 
        // self.whose =  Whose::Ours;
        self.color = Color::White; 
        self.en_passant = Square::Null; 
        self.half_moves = 0u8;
        self.rule50 = 0u8;
    }

    pub fn is_alpha(c : char) -> bool {
        return c.is_ascii_alphabetic();
    }

    pub fn is_lower(c: char) -> bool {
        return c.is_ascii_lowercase();
    }

    pub fn to_lower(c: char) -> char {
        if (Board::is_lower(c)) {
            return c;
        }
        else {
            return (c as u8 + 32) as char;
        }
    }

    pub fn board_from_fen_pieces(&mut self, pieces: &str) -> () {
        let mut pieces_len = pieces.len();

        let mut r = Rank::Eighth;
        let mut f = File::A;
        // let f = 0;
        let fen_slice = pieces.as_bytes();


        for i in 0..pieces_len {
            let piece_chr = fen_slice[i] as char;
            print!("piece_chr: {}\n", piece_chr);
            let piece = fen_slice[i] as i8;
            if 49 <= piece && piece <= 56 {
                let open = piece - 48;
                match open {
                    1 => f = File::convert(((f as i8) + open) as isize),
                    2 => f = File::convert(((f as i8) + open) as isize),
                    3 => f = File::convert(((f as i8) + open) as isize),
                    4 => f = File::convert(((f as i8) + open) as isize),
                    5 => f = File::convert(((f as i8) + open) as isize),
                    6 => f = File::convert(((f as i8) + open) as isize),
                    8 => f = File::convert(((f as i8) + open) as isize),
                    _ => panic!("{} is not better ", open)
                    
                }
                assert!((f as i8) <= 8);
            }
            else if Board::is_alpha(piece_chr) {
                let s = Square::from(f,r);
                // let b = s.to_bitboard();
                let is_black = Board::is_lower(piece_chr);
                match piece_chr {
                    'p' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::P)),
                    'n' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::N)),
                    'b' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::B)),
                    'r' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::R)),
                    'q' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::Q)),
                    'k' => Board::set(self,s, Piece::Pc(Whose::Theirs, PieceType::K)),

                    'P' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::P)),
                    'N' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::N)),
                    'B' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::B)),
                    'R' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::R)),
                    'Q' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::Q)),
                    'K' => Board::set(self,s, Piece::Pc(Whose::Ours, PieceType::K)),

                    _ => panic!("Alpha character {} is not a piece!", piece_chr)
                }

                // if (is_black) {

                // }
                // else {

                // }
                // print!("file1: {}\n", (f as i8) + 1i8);
                f = File::convert(((f as i8) + 1i8 ) as isize);
                // print!("file: {}\n", f as isize);
                assert!((f as i8) <= 8);
            }
            else if piece_chr == '/' {
                // print!("file234: {}\n", (f as i8));
                assert!((f as i8) == 8);
                f = File::A;
                r = Rank::convert(((r as i8) - 1) as isize);
            }
            else{
                panic!("Invalid character: {}", piece_chr);
            }
        }
    }

    pub fn board_from_fen(&mut self, fen: &str) -> () {
        // let P = Board::new();
        self.board_clear();
        let fenParts : Vec<_>= fen.split_whitespace().collect();
        let mut is_black = false;

        //Pieces
        Board::board_from_fen_pieces(self, fenParts[0]);
        print!("eijfoiwjefjiow\n");
        //Side to move
        let sideToMove= fenParts[1];
        if sideToMove.len() != 1 {
            panic!("Invalid FEN side-to-move {}", sideToMove)
        }
        if sideToMove == 'w'.to_string() {
            is_black = false
        }
        else if sideToMove == 'b'.to_string() {
            is_black = true
        }
        else {
            panic!("Invalid FEN side-to-move char: {}", sideToMove);
        }
        print!("1\n");

        //Castling
        let castling = fenParts[2];
        let castling_len = castling.len();
        if !(castling_len == 1 && castling == '-'.to_string()) {
            if (!(1 <= castling_len && castling_len <= 4)) {
                panic!("Castling string {} too long", castling);
            }
            let castlingChars: Vec<_>= castling.chars().collect();
            for c in castlingChars {
                let castlingElem = c;
                match castlingElem {
                    'K' => Board::castling_set(self, Whose::Ours, Side::K),
                    'Q' => Board::castling_set(self, Whose::Ours, Side::Q),
                    'k' => Board::castling_set(self, Whose::Theirs, Side::K),
                    'q' => Board::castling_set(self, Whose::Theirs, Side::Q),   
                    _ => panic!("Invalid castling char {}", castlingElem)               
                }
            }
        }

        //En passant flag
        let enPassant = fenParts[3];
        let mut enPassantFile = File::A;
        let mut enPassantRank = Rank::First;
        let mut enPassantChars: Vec<_> = enPassant.chars().collect();

        if enPassant != '-'.to_string() {
            enPassantFile = File::convert((enPassantChars[0] as i8) as isize);
            enPassantRank = Rank::convert((enPassantChars[1] as i8) as isize);
        }


        //Half moves
        let halfMove = fenParts[4];
        self.half_moves = (halfMove.parse::<i32>().unwrap()) as u8;

        //Full moves
        let fullMove = fenParts[5];
        self.rule50 = fullMove.parse::<i32>().unwrap() as u8;

        if is_black{
            Board::flip(self);
        }
        print!("2\n");

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
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut bd2 = Board::new();
        // bd2.whose_bbs = [Bitboard::EMPTY; Whose::COUNT];
        // bd2.piece_type_bbs = [Bitboard::EMPTY; PieceType::NK_COUNT];  
        bd2.board_from_fen(fen);
        bd2.print();
        print!("hi");
    }
}