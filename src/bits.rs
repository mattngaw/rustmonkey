use std::fmt::Display;
use std::ops;
use enum_iterator::IntoEnumIterator;
use super::util;

#[derive(IntoEnumIterator, PartialEq, Debug, Copy, Clone, Display)]
pub enum Rank { First, Second, Third, Fourth, Fifth, Sixth, Seventh, Eighth, }

#[derive(IntoEnumIterator, PartialEq, Debug, Copy, Clone, Display)]
pub enum File { A, B, C, D, E, F, G, H, }

#[derive(PartialEq, Debug, Display)]
pub enum Square {
    NullSq,
    Sq(u8),
}

impl Square {

    fn calculate(r: Rank, f: File) -> Square {
        Square::Sq((r as u8) * 8 + (f as u8))
    }

    fn rank(&self) -> Rank {
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

    fn file(&self) -> File {
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

    fn to_string(&self) {
        match self {
            Square::NullSq => panic!("Attempted to convert NullSq to string"),
            Square::Sq(s) => {
                let f: File = self.file();
                let r: Rank = self.rank();
                
            }
        }
    }
    
    fn flipped(&self) -> Square {
        match self {
            Square::NullSq => panic!("Attempted to flip a NullSq"),
            Square::Sq(s) => Square::Sq(63u8 - s),
        }
    }

    fn to_bitboard(&self) -> Bitboard {
        match self {
            Square::NullSq => panic!("Attempted to turn a NullSq to a Bb"),
            Square::Sq(s) => Bitboard::Bb(1 << s),
        }
    }
}

#[derive(PartialEq, Debug, Display, Clone, Copy)]
pub enum Bitboard {
    NullBb,
    Bb(u64),
}

impl Bitboard {
    const FULL: Bitboard = Bitboard::Bb(!0u64);
    const EMPTY: Bitboard = Bitboard::Bb(0u64);
    
    pub fn new(b: u64) -> Bitboard {
        Bitboard::Bb(b)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Bitboard::NullBb => panic!("Attempted to check if NullBb is empty"),
            Bitboard::Bb(b) => *b == 0u64,
        }
    }

    pub fn get(&self, s: Square) -> bool {
        match (*self, s) {
            (Bitboard::NullBb, _) => panic!("Attempted to get on NullBb"),
            (_, Square::NullSq) => panic!("Attempted to get with NullSq"),
            (B, s) => !((B & s.to_bitboard()).is_empty()),
        }
    }

    pub fn set(&mut self, s: &Square) {
        match (&self, s) {
            (Bitboard::NullBb, _) => panic!("Attempted to set on a NullBb"),
            (_, Square::NullSq) => panic!("Attempted to set with a NullSq"),
            (Bitboard::Bb(b), Square::Sq(s)) => {
                *self = Bitboard::Bb(*b | (1u64 << *s));
            }
        }
    }

    pub fn reset(&mut self, s: &Square) {
        match (&self, s) {
            (Bitboard::NullBb, _) => panic!("Attempted to reset on a NullBb"),
            (_, Square::NullSq) => panic!("Attempted to reset with a NullSq"),
            (Bitboard::Bb(b), Square::Sq(s)) => {
                *self = Bitboard::Bb(*b & !(1u64 << *s));
            }
        }
    }

    pub fn is_singular(&self) -> bool {
        match self {
            Bitboard::NullBb => panic!("Attempted to check if NullBb is singular"),
            Bitboard::Bb(b) => b.is_power_of_two(),
        }
    }

    pub fn to_square(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to turn a NullBb to Sq"),
            Bitboard::Bb(b) => {
                if self.is_singular() { Square::Sq(b.trailing_zeros() as u8) } 
                else { panic!("Attempted to turn non-singular Bb to Sq") }
            }
        }
    }

    pub fn pop_count(&self) -> u8 {
        match self {
            Bitboard::NullBb => panic!("Attempted to pop_count on NullBb"),
            Bitboard::Bb(b) => b.count_ones() as u8,
        }
    }

    pub fn lsb(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to get LSB of NullBb"),
            Bitboard::Bb(0u64) => Square::NullSq,
            Bitboard::Bb(b) => Square::Sq(b.trailing_zeros() as u8),
        }
    }

    pub fn msb(&self) -> Square {
        match self {
            Bitboard::NullBb => panic!("Attempted to get MSB of NullBb"),
            Bitboard::Bb(0u64) => Square::NullSq,
            Bitboard::Bb(b) => Square::Sq(b.leading_zeros() as u8),
        }
    }

    pub fn flip(&mut self) {
        match &self {
            Bitboard::NullBb => panic!("Attempted to pop_count on NullBb"),
            Bitboard::Bb(b) => *self = Bitboard::Bb(b.reverse_bits()),
        }
    }

    pub fn print(&self) {
        match self {
            Bitboard::NullBb => println!("NullBb"),
            B => {
                for row in util::PRINT_ORDER {
                    for i in row {
                        if B.get(Square::Sq(*i)) { print!("x "); } 
                        else { print!(". "); }
                    }
                    print!("\n");
                }
            }
        }
    }
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

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Bitboard::NullBb => panic!("Attempted to iterate "),
            Bitboard::Bb(0u64) => None,
            Bitboard::Bb(b) => {
                let lsb: Square = Square::Sq(b.trailing_zeros() as u8);
                self.reset(&lsb);
                Some(lsb)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(Square::Sq(11u8), Square::calculate(Rank::Second, File::D));
        
        let s1: Square = Square::Sq(15u8);
        assert_eq!(Square::Sq(48u8), s1.flipped());
    }

    #[test]
    fn test_bitboard() {
        let mut b1 = Bitboard::EMPTY;
        let s1: Square = Square::Sq(10u8);
        b1.set(&s1);
        assert_eq!(Bitboard::Bb(1024u64), b1);
        assert_eq!(Bitboard::new(1024u64), s1.to_bitboard());
        assert_eq!(Bitboard::new(1024u64), b1);
        assert!(b1.is_singular());
        b1.set(&Square::Sq(3u8));
        assert!(!b1.is_empty());
        assert!(!b1.is_singular());
        assert_eq!(Bitboard::new(1032u64), b1);
        assert_eq!(2, b1.pop_count());
        b1.reset(&Square::Sq(10u8));
        assert_eq!(Bitboard::new(8u64), b1);
        assert_eq!(Square::Sq(3u8), b1.to_square());
        assert_eq!(1, b1.pop_count());
        b1.flip();
        let b2 = Bitboard::new(!0u64);
        let b3 = Bitboard::new(!(1u64 << 60));
        assert_eq!(Square::Sq(60u8), b1.to_square());
        assert_eq!(b1, b1 & b2);
        assert_eq!(Bitboard::EMPTY, b1 & b3);
        b1.reset(&Square::Sq(60u8));
        assert!(b1.is_empty());
    }

}