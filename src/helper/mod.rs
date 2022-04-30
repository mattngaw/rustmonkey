//! An assortment of helper functions and constants

pub mod tables;

/// Movement directions on a chess board
pub enum Direction {
    North,
    East,
    South,
    West,
    Northeast,
    Southeast,
    Southwest,
    Northwest,
}