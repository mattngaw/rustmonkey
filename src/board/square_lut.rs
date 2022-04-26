//! A quick lookup table for determining what [`Piece`] is located at a square

use super::{Piece, Whose, PieceType};
use super::bits::Square;
use super::util::PRINT_ORDER;

/// A data-structure for quick square lookups
/// 
/// As opposed to performing bitwise operations on bitboards to figure out 
/// which piece is where
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct SquareLUT {
    data: [Piece; Square::COUNT],
}

impl SquareLUT {
    /// Creates a table where every square is empty
    pub fn new() -> SquareLUT {
        SquareLUT { 
            data: [Piece::Null; Square::COUNT],
        }
    }

    /// Gets the piece at the given square
    pub fn get(&self, sq: Square) -> Piece {
        match sq {
            Square::Null => panic!("Attempted to get from SquareLUT at Null"),
            Square::Sq(s) => self.data[s as usize],
        }
    }

    /// Sets the piece at the given square
    pub fn set(&mut self, sq: Square, p: Piece) -> () {
        match sq {
            Square::Null => panic!("Attempted to set in SquareLUT at Null"),
            Square::Sq(s) => self.data[s as usize] = p,
        }
    }

    // Flips a SquareLUT
    pub fn flip(&mut self) -> () {
        self.data.reverse();
        for p in &mut self.data {
            match p {
                Piece::Pc(w, _) => w.flip(),
                _ => ()
            }
        }
    }

    pub fn print(&self) {
        let mut board_string = String::new();
        for row in PRINT_ORDER {
            for i in row {
                let index = Square::Sq(*i);
                board_string.push(self.get(index).to_char());
                board_string.push(' ');
            }
            board_string.push('\n');
        }
        print!("{}", board_string);
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

/// Used to iterate over a SquareLUT
/// 
/// Iterates from Sq(0) to Sq(63)
#[derive(Copy, Clone, Debug)]
pub struct SquareLUTIntoIterator {
    lut: SquareLUT,
    index: Square,
}

impl Iterator for SquareLUTIntoIterator {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            Square::Null => {
                self.index = Square::Sq(0);
                None
            }
            Square::Sq(_) => {
                let result = Some(self.lut.get(self.index));
                self.index = self.index.next();
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    fn test_square_lut() {
        let mut sq_lut = SquareLUT::new();

        assert_eq!(sq_lut.get(Square::Sq(0u8)), Piece::Null);
        assert_eq!(sq_lut.get(Square::Sq(25u8)), Piece::Null);

        sq_lut.set(Square::Sq(0u8), Piece::Pc(Whose::Ours, PieceType::Q));
        assert_eq!(sq_lut.get(Square::Sq(0u8)),
                         Piece::Pc(Whose::Ours, PieceType::Q));
        assert_eq!(sq_lut.get(Square::Sq(25u8)), Piece::Null);
        sq_lut.flip();
        assert_eq!(sq_lut.get(Square::Sq(38u8)), Piece::Null);
        assert_eq!(sq_lut.get(Square::Sq(63u8)), Piece::Pc(Whose::Theirs, PieceType::Q));
    }
}