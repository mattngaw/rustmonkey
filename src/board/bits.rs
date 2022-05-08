//! The foundations of a chess board representation
//! 
//! A [Board](crate::board::Board) is made up [`Square`s](Square) and [`Bitboard`s](Bitboard).
//! 
//! # Example: Placing two kings on a bitboard
//! ```rust
//! // Create a square representing e1 (the white king)
//! let whiteKing = Square::from(File::E, Rank::First);
//! 
//! // Create a singular bitboard with the e1 square set
//! let mut b: Bitboard = whiteKing.to_bitboard();
//! 
//! // The same for e8 (the black king)
//! let blackKing = Square::from(File::E, Rank::Eighth);
//! 
//! // Set e8 on the bitboard
//! b.set(blackKing);
//! 
//! b.print();
//! /* stdout
//! . . . . x . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . . . . .
//! . . . . x . . .
//! */ 
//! ```

use std::ops;
use std::cmp::Ordering;
use std::fmt;

use super::util::*;

/// The rows of a chess board
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank { 
    First = 0,
    Second = 1, 
    Third = 2,
    Fourth = 3, 
    Fifth = 4, 
    Sixth = 5, 
    Seventh = 6, 
    Eighth = 7, 
    Null
}

impl Rank {
    /// Number of ranks
    const COUNT: usize = 8;

    /// Maps integers from zero to seven inclusive to ranks
    pub fn convert(x: isize) -> Rank {
        match x {
            0isize => Rank::First,
            1isize => Rank::Second,
            2isize => Rank::Third,
            3isize => Rank::Fourth,
            4isize => Rank::Fifth,
            5isize => Rank::Sixth,
            6isize => Rank::Seventh,
            7isize => Rank::Eighth,
            _ => Rank::Null
        }
    }

    /// Returns the flipped version of a rank
    pub fn flipped(&self) -> Rank {
        match *self {
            Rank::First => Rank::Eighth,
            Rank::Second => Rank::Seventh,
            Rank::Third => Rank::Sixth,
            Rank::Fourth => Rank::Fifth,
            Rank::Fifth => Rank::Fourth,
            Rank::Sixth => Rank::Third,
            Rank::Seventh => Rank::Second,
            Rank::Eighth => Rank::First,
            Rank::Null => Rank::Null
        }
    }
    // pub fn num_to_rank(num : u8) -> Rank {
    //     match num {
    //         1 => Rank::First,
    //         2 => Rank::Second,
    //         3 => Rank::Third,
    //         4 => Rank::Fourth,
    //         5 => Rank::Fifth,
    //         6 => Rank::Sixth,
    //         7 => Rank::Seventh,
    //         8 => Rank::Eighth,
    //         _ => Rank::Null
    //     }
    // }
}

/// The columns of a chess board
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum File { 
    A = 0, 
    B = 1, 
    C = 2, 
    D = 3, 
    E = 4, 
    F = 5, 
    G = 6, 
    H = 7, 
    Null
}

impl File {
    /// Number of files
    const COUNT: usize = 8;

    /// Maps integers from zero to seven inclusive to files
    pub fn convert(x: isize) -> File {
        match x {
            0isize => File::A,
            1isize => File::B,
            2isize => File::C,
            3isize => File::D,
            4isize => File::E,
            5isize => File::F,
            6isize => File::G,
            7isize => File::H,
            _ => File::Null
        }
    }

    /// Returns the flipped version of a file
    pub fn flipped(&self) -> File {
        match *self {
            File::A => File::H,
            File::B => File::G,
            File::C => File::F,
            File::D => File::E,
            File::E => File::D,
            File::F => File::C,
            File::G => File::B,
            File::H => File::A,
            File::Null => File::Null
        }
    }
    // pub fn num_to_file(num: u8) -> File {
    //     match num {
    //         1 => File::A,
    //         2 => File::B,
    //         3 => File::C,
    //         4 => File::D,
    //         5 => File::E,
    //         6 => File::F,
    //         7 => File::G,
    //         8 => File::H,
    //         _ => File::Null
    //     }
    // }
}

/// A value ranging from 0 to 64, representing the squares from a1-h8 in 
/// rank-major order
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Square {
    /// An invalid square (typically the result of unsafe operations)
    Null,
    
    /// A valid square (ranges from a1 to h8)
    Sq(u8),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Square::Null => write!(f, "Null"),
            Square::Sq(s) => write!(f, "Sq({})", s),
        }
    }
}

impl Square {
    /// Total number of squares in a board
    pub const COUNT: usize = 64;
    pub const MIN_VAL: u8 = 0;
    pub const MAX_VAL: u8 = 63;

    /// Creates a new `Square` given an integer `s`
    /// 
    /// Squares initialized with values greater than 63 are mapped to `Null`.
    pub fn new(s: u8) -> Square {
        if s > 63 { Square::Null } else { Square::Sq(s) }
    }

    /// Creates a new `Square` from a [`Rank`] and a [`File`]
    pub fn from(f: File, r: Rank) -> Square {
        match (f, r) {
            (File::Null, _) => Square::Null,
            (_, Rank::Null) => Square::Null,
            _ => Square::Sq((r as u8) * (Rank::COUNT as u8) + (f as u8))
        }
    }

    /// Unwraps the value of a square (assumes it has a value)
    pub fn val(&self) -> u8 {
        match *self {
            Square::Null => panic!("Expected to convert Square::Sq to value, instead got Square::Null"),
            Square::Sq(s) => s
        }
    }

    /// Checks if a square is Square::Null
    pub fn is_null(&self) -> bool {
        *self == Square::Null
    }

    /// Creates an iterator of squares from two squares
    pub fn range(sq1: Square, sq2: Square) -> SquareRange {
        if sq1.is_null() {
            panic!("Attempted to start range with null square");
        }
        if sq2.is_null() {
            panic!("Attempted to end range with null square");
        }
        let (s1, s2) = (sq1.val(), sq2.val());
        match s1.cmp(&s2) {
            Ordering::Equal => panic!("Attempted to make empty range"),
            _ => {
                SquareRange {
                    current: s1,
                    end: s2,
                }
            }
        }
    }

    /// Creates an iterator of squares from an integer range
    pub fn range_from_int(s1: u8, s2: u8) -> SquareRange {
        if s1 > Square::MAX_VAL {
            panic!("Attempted to start range with invalid square value");
        }
        if s2 != 64u8 && s2 > Square::MAX_VAL {
            panic!("Attempted to end range with invalid square value");
        }
        match s1.cmp(&s2) {
            Ordering::Equal => panic!("Attempted to make empty range"),
            _ => {
                SquareRange {
                    current: s1,
                    end: s2,
                }
            }
        }
    }

    /// Returns the [`Rank`] of the square
    pub fn rank(&self) -> Rank {
        match self {
            Square::Null => panic!("Attempted to get rank of Square::Null"),
            Square::Sq(s) => {
                match s / 8u8 {
                    0u8 => Rank::First,
                    1u8 => Rank::Second,
                    2u8 => Rank::Third,
                    3u8 => Rank::Fourth,
                    4u8 => Rank::Fifth,
                    5u8 => Rank::Sixth,
                    6u8 => Rank::Seventh,
                    7u8 => Rank::Eighth,
                    _ => panic!("Attempted to get rank of invalid square"),
                }
            }
        }
    }

    /// Returns the [`File`] of the square
    pub fn file(&self) -> File {
        match self {
            Square::Null => panic!("Attempted to get rank of Square::Null"),
            Square::Sq(s) => {
                match s % 8u8 {
                    0u8 => File::A,
                    1u8 => File::B,
                    2u8 => File::C,
                    3u8 => File::D,
                    4u8 => File::E,
                    5u8 => File::F,
                    6u8 => File::G,
                    7u8 => File::H,
                    _ => panic!("Attempted to get file of invalid square"),
                }
            }
        }
    }

    /// Returns the square above
    pub fn rank_up(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to +1 the rank of Square::Null"),
            Square::Sq(s) => {
                if self.rank() == Rank::Eighth { Square::Null }
                else { Square::Sq(s + 8) }
            }
        }
    }

    /// Returns the square below
    pub fn rank_down(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to -1 the rank of Square::Null"),
            Square::Sq(s) => {
                if self.rank() == Rank::First { Square::Null }
                else { Square::Sq(s - 8) }
            }
        }
    }

    /// Returns the square to the right
    pub fn file_up(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to +1 the file of Square::Null"),
            Square::Sq(s) => {
                if self.file() == File::H { Square::Null }
                else { Square::Sq(s + 1) }
            }
        }
    }

    /// Returns the square to the left
    pub fn file_down(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to -1 the file of Square::Null"),
            Square::Sq(s) => {
                if self.file() == File::A { Square::Null }
                else { Square::Sq(s - 1) }
            }
        }
    }

    /// Returns the next square
    pub fn next(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to get the next square after Square::Null"),
            Square::Sq(s) => {
                if s >= 63u8 { Square::Null }
                else { Square::Sq(s + 1) }
            }
        }
    }

    /// Returns the square offset by nmber of ranks and files
    pub fn offset(&self, dx: i8, dy: i8) -> Square {
        match *self {
            Square::Null => panic!("Attempted to get offset from Square::Null"),
            Square::Sq(s) => {
                let f = self.file();
                let r = self.rank();
                let f_val = (f as i8) + dx;
                let r_val = (r as i8) + dy;
                let f_ = File::convert(f_val as isize);
                let r_ = Rank::convert(r_val as isize);
                Square::from(f_, r_)
            }
        }
    }

    /// Returns the string representation of the square
    pub fn to_string(&self) -> String {
        match self {
            Square::Null => panic!("Attempted to convert Square::Null to string"),
            Square::Sq(_) => {
                let mut str: String = String::new();
                match self.file() {
                    File::A => str.push('a'),
                    File::B => str.push('b'),
                    File::C => str.push('c'),
                    File::D => str.push('d'),
                    File::E => str.push('e'),
                    File::F => str.push('f'),
                    File::G => str.push('g'),
                    File::H => str.push('h'),
                    File::Null => str.push('X')
                };
                match self.rank() {
                    Rank::First => str.push('1'),
                    Rank::Second => str.push('2'),
                    Rank::Third => str.push('3'),
                    Rank::Fourth => str.push('4'),
                    Rank::Fifth => str.push('5'),
                    Rank::Sixth => str.push('6'),
                    Rank::Seventh => str.push('7'),
                    Rank::Eighth => str.push('8'),
                    Rank::Null => str.push('X')
                }
                str
            }
        }
    }
    
    /// "Flips" representation of the square
    /// 
    /// Used when representing the chess board from the opponent's perspective.
    pub fn flip(&mut self) -> () {
        match *self {
            Square::Null => panic!("Attempted to flip a Square::Null"),
            Square::Sq(s) => *self = Square::Sq(63u8 - s),
        }
    }

    /// Returns the flipped square
    pub fn flipped(&self) -> Square {
        match *self {
            Square::Null => panic!("Attempted to get a Square::Null flipped"),
            Square::Sq(s) => Square::Sq(63u8 - s),
        }
    }

    /// Converts the square into a singular (only one set bit) [`Bitboard`]
    pub fn to_bitboard(&self) -> Bitboard {
        match self {
            Square::Null => panic!("Attempted to turn a Square::Null to a Bb"),
            Square::Sq(s) => Bitboard::Bb(1 << s),
        }
    }

    /// Prints the square
    pub fn print(&self) -> () {
        println!("{}", self.to_string());
    }
}

/// The IntoIterator made from two [Squares](Square)
#[derive(Copy, Clone, Debug)]
pub struct SquareRange {
    current: u8,
    end: u8
}

impl Iterator for SquareRange {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let result = Square::Sq(self.current);
            if self.current < self.end { self.current += 1 }
            else { self.current -= 1 } // self.current > self.end
            Some(result)
        }
    }
}

/// A 64-bit value, where each bit represents the occupancy of a square
/// 
/// The least-significant bit represents `Square(0)` (a1), and the 
/// most-significant bit represents `Square(63)` (h8).
/// 
/// The bits increase in rank-major order (i.e. the second LSB == `Square(1)` 
/// (b1), the third LSB == `Square(2)` (b2), etc.)
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Bitboard {
    /// An invalid bitboard (typically the result of an invalid operation)
    Null,

    Bb(u64),
}

impl ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a | b),
            (_, _) => panic!("Attempted to | with Bitboard::Null"),
        }
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a & b),
            (_, _) => panic!("Attempted to & with Bitboard::Null"),
        }
    }
}

impl ops::BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a ^ b),
            (_, _) => panic!("Attempted to ^ with Bitboard::Null"),
        }
    }
}

impl ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        match self {
            Bitboard::Null => panic!("Attempted to ! with Bitboard::Null"),
            Bitboard::Bb(a) => Bitboard::Bb(!a),
        }
    }
}

/// Creates an iterator out of the bitboard
/// 
/// Iterates over the next least significant bit
impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Bitboard::Null => panic!("Attempted to iterate over Bitboard::Null"),
            Bitboard::Bb(0u64) => None,
            Bitboard::Bb(b) => {
                let lsb: Square = Square::Sq(b.trailing_zeros() as u8);
                self.reset(lsb);
                Some(lsb)
            }
        }
    }
}

impl Bitboard {
    /// A bitboard with all bits set
    pub const FULL: Bitboard = Bitboard::Bb(!0u64);

    /// A bitboard with none of the bits set
    pub const EMPTY: Bitboard = Bitboard::Bb(0u64);
    
    /// Creates a new bitboard
    pub fn new(b: u64) -> Bitboard {
        Bitboard::Bb(b)
    }

    /// Returns whether or not the bitboard is empty (all zeros)
    pub fn is_empty(&self) -> bool {
        match self {
            Bitboard::Null => panic!("Attempted to check if Bitboard::Null is empty"),
            Bitboard::Bb(b) => *b == 0u64,
        }
    }

    /// Returns whether or not the `s`th bit is set
    pub fn get(&self, sq: Square) -> bool {
        match (*self, sq) {
            (Bitboard::Null, _) => panic!("Attempted to get on Bitboard::Null"),
            (_, Square::Null) => panic!("Attempted to get with Bitboard::Null"),
            (B, s) => !((B & sq.to_bitboard()).is_empty()),
        }
    }

    /// Sets the `s`th bit
    pub fn set(&mut self, sq: Square) -> () {
        match (&self, sq) {
            (Bitboard::Null, _) => panic!("Attempted to set on a Bitboard::Null"),
            (_, Square::Null) => panic!("Attempted to set with a Square::Null"),
            (Bitboard::Bb(b), Square::Sq(sq)) => {
                *self = Bitboard::Bb(*b | (1u64 << sq));
            }
        }
    }

    /// Resets the `s`th bit
    pub fn reset(&mut self, sq: Square) {
        match (&self, sq) {
            (Bitboard::Null, _) => panic!("Attempted to reset on a Bitboard::Null"),
            (_, Square::Null) => panic!("Attempted to reset with a Bitboard::Null"),
            (Bitboard::Bb(b), Square::Sq(sq)) => {
                *self = Bitboard::Bb(*b & !(1u64 << sq));
            }
        }
    }

    /// Returns whether or not the bitboard is singular
    pub fn is_singular(&self) -> bool {
        match self {
            Bitboard::Null => panic!("Attempted to check if Bitboard::Null is singular"),
            Bitboard::Bb(b) => b.is_power_of_two(),
        }
    }

    /// Converts a singular bitboard to a [`Square`]
    pub fn to_square(&self) -> Square {
        match self {
            Bitboard::Null => panic!("Attempted to turn a Bitboard::Null to Sq"),
            Bitboard::Bb(b) => {
                if self.is_singular() { Square::Sq(b.trailing_zeros() as u8) } 
                else { panic!("Attempted to turn non-singular Bb to Sq") }
            }
        }
    }

    /// Returns the number of set bits in the bitboard
    pub fn pop_count(&self) -> u8 {
        match self {
            Bitboard::Null => panic!("Attempted to pop_count on Bitboard::Null"),
            Bitboard::Bb(b) => b.count_ones() as u8,
        }
    }

    /// Returns the least-significant set bit as a [`Square`]
    pub fn lsb(&self) -> Square {
        match self {
            Bitboard::Null => panic!("Attempted to get LSB of Bitboard::Null"),
            Bitboard::Bb(0u64) => Square::Null,
            Bitboard::Bb(b) => Square::Sq(b.trailing_zeros() as u8),
        }
    }

    /// Returns the most-significant set bit as a [`Square`]
    pub fn msb(&self) -> Square {
        match self {
            Bitboard::Null => panic!("Attempted to get MSB of Bitboard::Null"),
            Bitboard::Bb(0u64) => Square::Null,
            Bitboard::Bb(b) => Square::Sq(b.leading_zeros() as u8),
        }
    }

    /// Flips the bitboard
    /// 
    /// Used to represent the bitboard for the opponent's perspective. 
    pub fn flip(&mut self) {
        match *self {
            Bitboard::Null => panic!("Attempted to pop_count on Bitboard::Null"),
            Bitboard::Bb(b) => *self = Bitboard::Bb(b.reverse_bits()),
        }
    }

    /// Prints the bitboard as an 8x8 grid
    pub fn print(&self) {
        match self {
            Bitboard::Null => println!("Bitboard::Null"),
            B => {
                let mut bb_string = String::new();
                for row in PRINT_ORDER {
                    for i in row {
                        if B.get(Square::Sq(*i)) { 
                            bb_string.push_str("x ");
                        } 
                        else {
                            bb_string.push_str(". ");
                        }
                    }
                    bb_string.push('\n');
                }
                println!("{}", bb_string);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    // #[test]
    fn test_square() {
        assert_eq!(Square::Sq(11u8), Square::from(File::D, Rank::Second));
        
        let mut s1: Square = Square::Sq(15u8);
        s1.flip();
        assert_eq!(Square::Sq(48u8), s1);
    }

    // #[test]
    fn test_bitboard() {
        let mut b1 = Bitboard::EMPTY;
        let s1: Square = Square::Sq(10u8);
        b1.set(s1);
        assert_eq!(Bitboard::Bb(1024u64), b1);
        assert_eq!(Bitboard::new(1024u64), s1.to_bitboard());
        assert_eq!(Bitboard::new(1024u64), b1);
        assert!(b1.is_singular());
        b1.set(Square::Sq(3u8));
        assert_eq!(Square::Sq(3u8).to_string(), "d1");
        assert!(!b1.is_empty());
        assert!(!b1.is_singular());
        assert_eq!(Bitboard::new(1032u64), b1);
        assert_eq!(2, b1.pop_count());
        b1.reset(Square::Sq(10u8));
        assert_eq!(Bitboard::new(8u64), b1);
        assert_eq!(Square::Sq(3u8), b1.to_square());
        assert_eq!(1, b1.pop_count());
        b1.flip();
        let b2 = Bitboard::new(!0u64);
        let b3 = Bitboard::new(!(1u64 << 60));
        assert_eq!(Square::Sq(60u8), b1.to_square());
        assert_eq!(Square::Sq(60u8).to_string(), "e8");
        assert_eq!(b1, b1 & b2);
        assert_eq!(Bitboard::EMPTY, b1 & b3);
        b1.reset(Square::Sq(60u8));
        assert!(b1.is_empty());
    }

    // #[test]
    fn test_offset() {
        assert_eq!(Square::from(File::E, Rank::Fourth).offset(2, 1),
                   Square::from(File::G, Rank::Fifth));
        assert_eq!(Square::from(File::E, Rank::Fourth).offset(5, 1),
                   Square::Null);

    }

}
