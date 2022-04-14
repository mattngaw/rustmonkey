//! A quick lookup table for determining what [`Piece`] is located at a square

use std::cmp::Ordering;
use std::ops::{Index, IndexMut};
use std::fmt::Display;

use super::{Piece, Whose, PieceType};
use super::bits::Square;
use super::util::PRINT_ORDER;

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

impl Display for SquareLUT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut board_string = String::new();
        for row in PRINT_ORDER {
            for i in row {
                let index = Square::Sq(*i);
                board_string.push(self[index].to_char());
                board_string.push(' ');
            }
            board_string.push('\n');
        }
        write!(f, "{}", board_string)
    }
}

impl SquareLUT {
    /// Creates a table where every square is empty
    pub fn new() -> SquareLUT {
        SquareLUT { 
            data: [Piece::Empty; Square::COUNT],
        }
    }
}

impl IntoIterator for SquareLUT {
    type Item = Piece;
    type IntoIter = SquareLUTIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SquareLUTIntoIterator {
            lut: self,
            index: Square::Sq(0),
        }
    }
}

pub struct SquareLUTIntoIterator {
    lut: SquareLUT,
    index: Square,
}

impl Iterator for SquareLUTIntoIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            Square::NullSq => {
                self.index = Square::Sq(0);
                None
            }
            Square::Sq(_) => {
                let result = Some(self.lut[self.index]);
                self.index = self.index.next();
                result
            }
        }
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

        let mut sq_lut = SquareLUT::new();
        sq_lut[Square::Sq(0u8)] = Piece::Pc(Whose::Ours, PieceType::R);
        sq_lut[Square::Sq(1u8)] = Piece::Pc(Whose::Ours, PieceType::N);
        sq_lut[Square::Sq(2u8)] = Piece::Pc(Whose::Ours, PieceType::B);
        sq_lut[Square::Sq(3u8)] = Piece::Pc(Whose::Ours, PieceType::Q);
        sq_lut[Square::Sq(4u8)] = Piece::Pc(Whose::Ours, PieceType::K);
        sq_lut[Square::Sq(5u8)] = Piece::Pc(Whose::Ours, PieceType::B);
        sq_lut[Square::Sq(6u8)] = Piece::Pc(Whose::Ours, PieceType::N);
        sq_lut[Square::Sq(7u8)] = Piece::Pc(Whose::Ours, PieceType::R);
        sq_lut[Square::Sq(8u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(9u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(10u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(11u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(28u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(13u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(14u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(15u8)] = Piece::Pc(Whose::Ours, PieceType::P);
        sq_lut[Square::Sq(48u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(49u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(50u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(51u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(52u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(53u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(54u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(55u8)] = Piece::Pc(Whose::Theirs, PieceType::P);
        sq_lut[Square::Sq(56u8)] = Piece::Pc(Whose::Theirs, PieceType::R);
        sq_lut[Square::Sq(57u8)] = Piece::Pc(Whose::Theirs, PieceType::N);
        sq_lut[Square::Sq(58u8)] = Piece::Pc(Whose::Theirs, PieceType::B);
        sq_lut[Square::Sq(59u8)] = Piece::Pc(Whose::Theirs, PieceType::Q);
        sq_lut[Square::Sq(60u8)] = Piece::Pc(Whose::Theirs, PieceType::K);
        sq_lut[Square::Sq(61u8)] = Piece::Pc(Whose::Theirs, PieceType::B);
        sq_lut[Square::Sq(62u8)] = Piece::Pc(Whose::Theirs, PieceType::N);
        sq_lut[Square::Sq(63u8)] = Piece::Pc(Whose::Theirs, PieceType::R);
        
        /*
         * Should print this:
         * r n b q k b n r
         * p p p p p p p p
         * . . . . . . . .
         * . . . . . . . .
         * . . . . P . . .
         * . . . . . . . .
         * P P P P . P P P
         * R N B Q K B N R
         */

        println!("{}", sq_lut);
    }

}