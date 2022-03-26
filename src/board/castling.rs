//! Keeps track of castling rights

use super::*;
use std::ops;
use ux::*;

/// Represents the two sides where one can castle
#[derive(Debug, PartialEq)]
pub enum CastlingSide { Kingside, Queenside }

/// A 4-bit word representing who still has castling rights
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Castling(u4);

impl Castling {
    /// Both sides can still castle either way
    const FULL: Castling = Castling(u4::new(0b1111));

    /// Both sides cannot castle either way
    const EMPTY: Castling = Castling(u4::new(0b0000));

    /// Creates a new `Castling`
    pub fn new(v: u8) -> Castling {
        Castling(u4::new(v))
    }

    /// Returns a flag representing the castling rights for one player 
    /// and one side
    pub fn flag(w: Whose, cs: CastlingSide) -> Castling {
        if w == Whose::Ours {
            if cs == CastlingSide::Kingside { 
                return Castling(u4::new(0b1000));
            } else { 
                return Castling(u4::new(0b0100));
            }
        } else { // w == Whose::Theirs
            if cs == CastlingSide::Kingside { 
                return Castling(u4::new(0b0010));
            } else { 
                return Castling(u4::new(0b0001));
            }
        }
    }
}

impl ops::BitOr for Castling {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitAnd for Castling {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_flags() {
        assert_eq!(Castling::flag(Whose::Ours, CastlingSide::Kingside), 
                   Castling::new(0b1000));
    }

}

