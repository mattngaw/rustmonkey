use crate::board::bits::{File, Rank, Square, Bitboard};

static mut FILES_TABLE: [Bitboard; 8] = [Bitboard::Null; 8];

static mut RANKS_TABLE: [Bitboard; 8] = [Bitboard::Null; 8];

static mut PAWN_MOVES_TABLE: [Bitboard; 64] = [Bitboard::Null; 64];

static mut PAWN_ATTACKS_TABLE: [Bitboard; 64] = [Bitboard::Null; 64];

static mut KNIGHT_TABLE: [Bitboard; 64] = [Bitboard::Null; 64];

static mut KING_TABLE: [Bitboard; 64] = [Bitboard::Null; 64];

static mut RAYS_TABLE: [[Bitboard; 64]; 8] = [[Bitboard::Null; 64]; 8];

static KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1,2), (2,1), (2,-1), (1,-2), (-1,-2), (-2,-1), (-2,1), (-1,2)
];

static KING_OFFSETS: [(i8, i8); 8] = [
    (0,1), (1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1)
];

pub fn get_file_bb(f: File) -> Bitboard {
    unsafe {
        match f {
            File::Null => panic!("Attempted to get file bitboard from File::Null"),
            _ => FILES_TABLE[f as usize]
        }
    }
}

pub fn get_rank_bb(r: Rank) -> Bitboard {
    unsafe {
        match r {
            Rank::Null => panic!("Attempted to get rank bitboard from Rank::Null"),
            _ => RANKS_TABLE[r as usize]
        }
    }
}

pub fn get_pawn_moves(sq: Square) -> Bitboard {
    unsafe {
        match sq {
            Square::Null => panic!("Attempted to get pawn moves from Square::Null"),
            Square::Sq(s) => PAWN_MOVES_TABLE[s as usize]
        }
    }
}

pub fn get_pawn_attacks(sq: Square) -> Bitboard {
    unsafe {
        match sq {
            Square::Null => panic!("Attempted to get pawn attacks from Square::Null"),
            Square::Sq(s) => PAWN_ATTACKS_TABLE[s as usize]
        }
    }
}

pub fn get_knight_moves(sq: Square) -> Bitboard {
    unsafe {
        match sq {
            Square::Null => panic!("Attempted to get knight moves from Square::Null"),
            Square::Sq(s) => KNIGHT_TABLE[s as usize]
        }
    }
}

pub fn get_king_moves(sq: Square) -> Bitboard {
    unsafe {
        match sq {
            Square::Null => panic!("Attempted to get king moves from Square::Null"),
            Square::Sq(s) => KING_TABLE[s as usize]
        }
    }
}

fn build_files_table() -> () {
    unsafe {
        for (i, bb) in FILES_TABLE.iter_mut().enumerate() {
            let f = File::convert(i as isize);
            *bb = Bitboard::EMPTY;
            for r_value in 0..8 {
                let r = Rank::convert(r_value as isize);
                let sq = Square::from(f, r);
                match sq {
                    Square::Null => (),
                    Square::Sq(_) => bb.set(sq),
                }
            }
        }
    }
}

fn build_ranks_table() -> () {
    unsafe {
        for (i, bb) in RANKS_TABLE.iter_mut().enumerate() {
            let r = Rank::convert(i as isize);
            *bb = Bitboard::EMPTY;
            for f_value in 0..8 {
                let f = File::convert(f_value as isize);
                let sq = Square::from(f, r);
                match sq {
                    Square::Null => (),
                    Square::Sq(_) => bb.set(sq),
                }
            }
        }
    }
}

fn build_pawn_moves_table() -> () {
    unsafe {
        for (i, bb) in PAWN_MOVES_TABLE.iter_mut().enumerate() {
            let sq = Square::new(i as u8);
            *bb = Bitboard::EMPTY;
            match sq.rank() {
                Rank::First | Rank::Eighth => continue,
                Rank::Second => {
                    bb.set(sq.rank_up());
                    bb.set(sq.rank_up().rank_up());
                }
                _ => bb.set(sq.rank_up())
            }
        }
    }
}

fn build_pawn_attacks_table() -> () {
    unsafe {
        for (i, bb) in PAWN_ATTACKS_TABLE.iter_mut().enumerate() {
            let sq = Square::new(i as u8);
            *bb = Bitboard::EMPTY;
            match (sq.rank(), sq.file()) {
                (Rank::First, _) | (Rank::Eighth, _) => continue,
                (_, File::A) => bb.set(sq.rank_up().file_up()),
                (_, File::H) => bb.set(sq.rank_up().file_down()),
                (_, _) => {
                    bb.set(sq.rank_up().file_up());
                    bb.set(sq.rank_up().file_down());
                }
            }
        }
    }
}

fn build_knight_moves() -> () {
    unsafe {
        for (i, bb) in KNIGHT_TABLE.iter_mut().enumerate() {
            let sq = Square::new(i as u8);
            *bb = Bitboard::EMPTY;
            for (dx, dy) in KNIGHT_OFFSETS {
                let sq_ = sq.offset(dx, dy);
                match sq_ {
                    Square::Null => (),
                    Square::Sq(_) => bb.set(sq_),
                }
            }
        }
    }
}

fn build_king_moves() -> () {
    unsafe {
        for (i, bb) in KING_TABLE.iter_mut().enumerate() {
            let sq = Square::new(i as u8);
            *bb = Bitboard::EMPTY;
            for (dx, dy) in KING_OFFSETS {
                let sq_ = sq.offset(dx, dy);
                match sq_ {
                    Square::Null => (),
                    Square::Sq(_) => bb.set(sq_),
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bits::*;

    // #[test]
    fn test_files() {
        build_files_table();
        get_file_bb(File::A).print();
        get_file_bb(File::E).print();
    }

    // #[test]
    fn test_ranks() {
        build_ranks_table();
        get_rank_bb(Rank::First).print();
        get_rank_bb(Rank::Fourth).print();
    }

    // #[test]
    fn test_pawn_moves() {
        build_pawn_moves_table();
        get_pawn_moves(Square::from(File::E, Rank::Fourth)).print();
        get_pawn_moves(Square::from(File::B, Rank::Second)).print();
        get_pawn_moves(Square::from(File::A, Rank::First)).print();
    }

    // #[test]
    fn test_pawn_attacks() {
        build_pawn_attacks_table();
        get_pawn_attacks(Square::from(File::E, Rank::Fourth)).print();
        get_pawn_attacks(Square::from(File::B, Rank::Second)).print();
        get_pawn_attacks(Square::from(File::A, Rank::First)).print();
        get_pawn_attacks(Square::from(File::A, Rank::Third)).print();
        get_pawn_attacks(Square::from(File::H, Rank::Seventh)).print();
    }

    // #[test]
    fn test_knights() {
        build_knight_moves();
        get_knight_moves(Square::from(File::E, Rank::Fourth)).print();
        get_knight_moves(Square::from(File::H, Rank::Seventh)).print();
    }

    // #[test]
    fn test_kings() {
        build_king_moves();
        get_king_moves(Square::from(File::E, Rank::Fourth)).print();
        get_king_moves(Square::from(File::H, Rank::Seventh)).print();
    }
}

