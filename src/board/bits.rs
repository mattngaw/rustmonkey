//! The foundations of a chess board representation
//! 
//! A [Position] is made up [`Square`s](Square) and [`Bitboard`s](Bitboard).
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

use super::util::*;

/// The rows of a chess board
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Rank { First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth, }

impl Rank {
    const COUNT: usize = 8;
}

/// The columns of a chess board
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum File { A, B, C, D, E, F, G, H, }

impl File {
    const COUNT: usize = 8;
}

/// A value ranging from 0 to 64, representing the squares from a1-h8 in 
/// rank-major order
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Square {
    /// An invalid square (typically the result of unsafe operations)
    NullSq,
    
    /// A valid square (ranges from a1 to h8)
    Sq(u8),
}

impl Square {
    /// Total number of squares in a board
    pub const COUNT: usize = 64;

    /// Creates a new `Square` given an integer `s`
    /// 
    /// Squares initialized with values greater than 63 are mapped to `NullSq`.
    pub fn new(s: u8) -> Square {
        if s > 63 { Square::NullSq } else { Square::Sq(s) }
    }

    /// Creates a new `Square` from a [`Rank`] and a [`File`]
    pub fn from(f: File, r: Rank) -> Square {
        Square::Sq((r as u8) * (Rank::COUNT as u8) + (f as u8))
    }

    /// Returns the [`Rank`] of the square
    pub fn rank(&self) -> Rank {
        match self {
            Square::NullSq => panic!("Attempted to get rank of NullSq"),
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
            Square::NullSq => panic!("Attempted to get rank of NullSq"),
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

    pub fn rank_up(&self) -> Square {
        match *self {
            Square::NullSq => panic!("Attempted to +1 the rank of NullSq"),
            Square::Sq(s) => {
                if self.rank() == Rank::Eighth { Square::Sq(s) }
                else { Square::Sq(s + 8) }
            }
        }
    }

    pub fn rank_down(&self) -> Square {
        match *self {
            Square::NullSq => panic!("Attempted to -1 the rank of NullSq"),
            Square::Sq(s) => {
                if self.rank() == Rank::First { Square::Sq(s) }
                else { Square::Sq(s - 8) }
            }
        }
    }

    pub fn file_up(&self) -> Square {
        match *self {
            Square::NullSq => panic!("Attempted to +1 the file of NullSq"),
            Square::Sq(s) => {
                if self.file() == File::H { Square::Sq(s) }
                else { Square::Sq(s + 1) }
            }
        }
    }

    pub fn file_down(&self) -> Square {
        match *self {
            Square::NullSq => panic!("Attempted to -1 the file of NullSq"),
            Square::Sq(s) => {
                if self.file() == File::A { Square::Sq(s) }
                else { Square::Sq(s - 1) }
            }
        }
    }

    pub fn next(&self) -> Square {
        match *self {
            Square::NullSq => panic!(""),
            Square::Sq(s) => {
                if s >= 63u8 { Square::NullSq }
                else { Square::Sq(s + 1) }
            }
        }
    }

    /// Returns the string representation of the square
    pub fn to_string(&self) -> String{ 
        match self {
            Square::NullSq => panic!("Attempted to convert NullSq to string"),
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
            Square::NullSq => panic!("Attempted to flip a NullSq"),
            Square::Sq(s) => *self = Square::Sq(63u8 - s),
        }
    }

    /// Converts the square into a singular (only one set bit) [`Bitboard`]
    pub fn to_bitboard(&self) -> Bitboard {
        match self {
            Square::NullSq => panic!("Attempted to turn a NullSq to a Bb"),
            Square::Sq(s) => Bitboard::Bb(1 << s),
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
    NullBb,

    Bb(u64),
}

impl ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a | b),
            (_, _) => panic!("Attempted to | with NullBb"),
        }
    }
}

impl ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a & b),
            (_, _) => panic!("Attempted to & with NullBb"),
        }
    }
}

impl ops::BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Bitboard {
        match (self, rhs) {
            (Bitboard::Bb(a), Bitboard::Bb(b)) => Bitboard::Bb(a ^ b),
            (_, _) => panic!("Attempted to ^ with NullBb"),
        }
    }
}

impl ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Bitboard {
        match self {
            Bitboard::NullBb => panic!("Attempted to ! with NullBb"),
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
            Bitboard::NullBb => panic!("Attempted to iterate "),
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
            Bitboard::NullBb => panic!("Attempted to check if NullBb is empty"),
            Bitboard::Bb(b) => *b == 0u64,
        }
    }

    /// Returns whether or not the `s`th bit is set
    pub fn get(&self, sq: Square) -> bool {
        match (*self, sq) {
            (Bitboard::NullBb, _) => panic!("Attempted to get on NullBb"),
            (_, Square::NullSq) => panic!("Attempted to get with NullSq"),
            (B, s) => !((B & sq.to_bitboard()).is_empty()),
        }
    }

    /// Sets the `s`th bit
    pub fn set(&mut self, sq: Square) -> () {
        match (&self, sq) {
            (Bitboard::NullBb, _) => panic!("Attempted to set on a NullBb"),
            (_, Square::NullSq) => panic!("Attempted to set with a NullSq"),
            (Bitboard::Bb(b), Square::Sq(sq)) => {
                *self = Bitboard::Bb(*b | (1u64 << sq));
            }
        }
    }

    /// Resets the `s`th bit
    pub fn reset(&mut self, sq: Square) {
        match (&self, sq) {
            (Bitboard::NullBb, _) => panic!("Attempted to reset on a NullBb"),
            (_, Square::NullSq) => panic!("Attempted to reset with a NullSq"),
            (Bitboard::Bb(b), Square::Sq(sq)) => {
                *self = Bitboard::Bb(*b & !(1u64 << sq));
            }
        }
    }

    /// Returns whether or not the bitboard is singular
    pub fn is_singular(&self) -> bool {
        match self {
            Bitboard::NullBb => panic!("Attempted to check if NullBb is singular"),
            Bitboard::Bb(b) => b.is_power_of_two(),
        }
    }

    /// Converts a singular bitboard to a [`Square`]
    pub fn to_square(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to turn a NullBb to Sq"),
            Bitboard::Bb(b) => {
                if self.is_singular() { Square::Sq(b.trailing_zeros() as u8) } 
                else { panic!("Attempted to turn non-singular Bb to Sq") }
            }
        }
    }

    /// Returns the number of set bits in the bitboard
    pub fn pop_count(&self) -> u8 {
        match self {
            Bitboard::NullBb => panic!("Attempted to pop_count on NullBb"),
            Bitboard::Bb(b) => b.count_ones() as u8,
        }
    }

    /// Returns the least-significant set bit as a [`Square`]
    pub fn lsb(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to get LSB of NullBb"),
            Bitboard::Bb(0u64) => Square::NullSq,
            Bitboard::Bb(b) => Square::Sq(b.trailing_zeros() as u8),
        }
    }

    /// Returns the most-significant set bit as a [`Square`]
    pub fn msb(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to get MSB of NullBb"),
            Bitboard::Bb(0u64) => Square::NullSq,
            Bitboard::Bb(b) => Square::Sq(b.leading_zeros() as u8),
        }
    }

    /// Flips the bitboard
    /// 
    /// Used to represent the bitboard for the opponent's perspective. 
    pub fn flip(&mut self) {
        match *self {
            Bitboard::NullBb => panic!("Attempted to pop_count on NullBb"),
            Bitboard::Bb(b) => *self = Bitboard::Bb(b.reverse_bits()),
        }
    }

    /// Prints the bitboard as an 8x8 grid
    pub fn print(&self) {
        match self {
            Bitboard::NullBb => println!("NullBb"),
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

    #[test]
    fn test_square() {
        assert_eq!(Square::Sq(11u8), Square::from(File::D, Rank::Second));
        
        let mut s1: Square = Square::Sq(15u8);
        s1.flip();
        assert_eq!(Square::Sq(48u8), s1);
    }

    #[test]
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

}
