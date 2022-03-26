//! A quick lookup table for determining what [`Piece`] is located at a square

use std::ops::{Index, IndexMut};

use super::{Piece, Whose, PieceType};
use super::bits::Square;

/// An array of 64 [`Piece`]s
#[derive(Debug)]
pub struct SquareLUT {
    data: [Piece; Square::COUNT],
}

impl Index<Square> for SquareLUT {
    type Output = Piece;

    fn index(&self, sq: Square) -> &Self::Output {
        match sq {
            Square::NullSq => &Piece::NullPc,
            Square::Sq(s) => &self.data[s as usize],
        }
    }
}

impl IndexMut<Square> for SquareLUT {
    fn index_mut(&mut self, sq: Square) -> &mut Self::Output {
        match sq {
            Square::NullSq => panic!("Attempted to assign with a NullSq index"),
            Square::Sq(s) => &mut self.data[s as usize],
        }
    }
}

impl SquareLUT {
    /// Creates a table where every square is empty
    pub fn new() -> SquareLUT {
        SquareLUT { data: [Piece::Empty; Square::COUNT] }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_square_lut() {
        let mut sq_lut = SquareLUT::new();

        assert_eq!(sq_lut[Square::Sq(0u8)], Piece::Empty);
        assert_eq!(sq_lut[Square::Sq(25u8)], Piece::Empty);

        sq_lut[Square::Sq(0u8)] = Piece::Pc(Whose::Ours, PieceType::Q);
        assert_eq!(sq_lut[Square::Sq(0u8)],
                         Piece::Pc(Whose::Ours, PieceType::Q));
        assert_eq!(sq_lut[Square::Sq(25u8)], Piece::Empty);
    }

}