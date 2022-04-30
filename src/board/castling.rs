//! Keeps track of castling rights

use super::*;

/// A 4-bit word representing who still has castling rights
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Castling(u8);

impl Default for Castling {
    fn default() -> Self { Castling(0u8) }
}

impl Castling {
    /// Both sides can still castle either way
    pub const FULL: Castling = Castling(0b1111u8);

    /// Both sides cannot castle either way
    pub const EMPTY: Castling = Castling(0b0000u8);

    /// Creates a new `Castling`
    pub fn new(v: u8) -> Castling {
        Castling(v)
    }

    /// Gets the castling rights for a side and player
    pub fn get(&mut self, w: Whose, cs: Side) -> bool {
        let Castling(v) = *self;
        match (w, cs) {
            (Whose::Ours, Side::K) => {
                (v & 0b1000u8) == 0b1000u8
            }
            (Whose::Ours, Side::Q) => {
                (v & 0b0100u8) == 0b0100u8
            }
            (Whose::Theirs, Side::K) => {
                (v & 0b0010u8) == 0b0010u8
            }
            (Whose::Theirs, Side::Q) => {
                (v & 0b0001u8) == 0b0001u8
            }
        }
    }

    /// Sets the castling rights for a side and player
    pub fn set(&mut self, w: Whose, cs: Side) -> () {
        let Castling(v) = *self;
        match (w, cs) {
            (Whose::Ours, Side::K) => {
                *self = Castling(v | 0b1000u8);
            }
            (Whose::Ours, Side::Q) => {
                *self = Castling(v | 0b0100u8);
            }
            (Whose::Theirs, Side::K) => {
                *self = Castling(v | 0b0010u8);
            }
            (Whose::Theirs, Side::Q) => {
                *self = Castling(v | 0b0001u8);
            }
        }
    }

    /// Resets the castling rights for a side and player
    pub fn reset(&mut self, w: Whose, cs: Side) -> () {
        let Castling(v) = *self;
        match (w, cs) {
            (Whose::Ours, Side::K) => {
                *self = Castling(v & 0b0111u8);
            }
            (Whose::Ours, Side::Q) => {
                *self = Castling(v & 0b1011u8);
            }
            (Whose::Theirs, Side::K) => {
                *self = Castling(v & 0b1101u8);
            }
            (Whose::Theirs, Side::Q) => {
                *self = Castling(v & 0b1110u8);
            }
        }
    }

    /// Flips the castling flags such that Our flags become Theirs and Theirs
    /// Ours
    pub fn flip(&mut self) -> () {
        let Castling(val) = *self;
        let our_new = (val & 0b1100u8) >> 2;
        let their_new = (val & 0b0011u8) << 2;
        *self = Castling(our_new | their_new);
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    // #[test]
    fn test_get_set_reset() {
        let mut c = Castling::new(0u8);
        assert_eq!(c, Castling::default());
        c.set(Whose::Ours, Side::K);
        assert_eq!(c, Castling(0b1000u8));
        assert_eq!(c.get(Whose::Ours, Side::K), true);
        assert_eq!(c.get(Whose::Theirs, Side::Q), false);
        c.reset(Whose::Ours, Side::K);
        assert_eq!(c.get(Whose::Ours, Side::K), false);
    }

}

