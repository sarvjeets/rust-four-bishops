#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: u8,  // The x cordinate, 0 <= x < 5
    y: u8,  // The y coordinate 0 <= x < 4
}

impl Position {
    fn new(x: u8, y: u8) -> Position {
        assert!(x < 5);
        assert!(y < 4);
        Position {x, y}
    }

    fn to_u32(self) -> u32 {
        (4 * self.x + self.y) as u32
    }

    fn from_u32(pos: u32) -> Self {
        Position::new((pos / 4).try_into().unwrap(),
                      (pos % 4).try_into().unwrap())
    }
}

struct Board {
    white_pos: [Position; 4],
    black_pos: [Position; 4],
}

#[derive(Debug, PartialEq)]
pub enum Piece {
    White,
    Black,
    Empty
}

impl Board {
    // Creates a new Board with pieces in initial positions.
    fn new() -> Self {
        let y_pos : [u8; 4] = [0, 1, 2, 3];
        let white_pos : [Position; 4] = y_pos.into_iter().map(
            |y| Position::new(0, y)).collect::<Vec<Position>>()
            .try_into().unwrap();
        let black_pos : [Position; 4] = y_pos.into_iter().map(
            |y| Position::new(4, y)).collect::<Vec<Position>>()
            .try_into().unwrap();
        Board::from_pos(white_pos, black_pos)
    }

    fn from_pos(white_pos: [Position; 4], black_pos: [Position; 4]) -> Self {
        Board{white_pos, black_pos}
    }

    fn to_u32(self) -> u32 {
        let mut white_even_pos = 0u32;
        let mut white_odd_pos = 0u32;
        let mut black_even_pos = 0u32;
        let mut black_odd_pos = 0u32;

        for position in self.white_pos {
            let pos = position.to_u32();
            if pos % 2 == 0 {
                white_even_pos = (white_even_pos << 4) | (pos / 2);
            } else {
                white_odd_pos = (white_odd_pos << 4) | (pos / 2);
            }
        }

        for position in self.black_pos {
            let pos = position.to_u32();
            if pos % 2 == 0 {
                black_even_pos = (black_even_pos << 4) | (pos / 2);
            } else {
                black_odd_pos = (black_odd_pos << 4) | (pos / 2);
            }
        }

        white_even_pos << 24 | white_odd_pos << 16
        | black_even_pos << 8 | black_odd_pos
    }

    fn from_u32(encoded: u32) -> Self {
        let mask_8bit = 0x000000FFu32;
        let white_even_pos = encoded >> 24;
        let white_odd_pos = (encoded & (mask_8bit << 16)) >> 16;
        let black_even_pos = (encoded & (mask_8bit << 8)) >> 8;
        let black_odd_pos = encoded & mask_8bit;

        let mask_4bit = 0x0000000Fu32;
        let white_pos = [
            Position::from_u32((white_even_pos >> 4) * 2),
            Position::from_u32((white_even_pos & mask_4bit) * 2),
            Position::from_u32((white_odd_pos >> 4) * 2 + 1),
            Position::from_u32((white_odd_pos & mask_4bit) * 2 + 1)];

        let black_pos = [
            Position::from_u32((black_even_pos >> 4) * 2),
            Position::from_u32((black_even_pos & mask_4bit) * 2),
            Position::from_u32((black_odd_pos >> 4) * 2 + 1),
            Position::from_u32((black_odd_pos & mask_4bit) * 2 + 1)];
        Board::from_pos(white_pos, black_pos)
    }

    fn get_piece(&self, given_pos: Position) -> Piece {
        for pos in self.white_pos.iter() {
            if given_pos == *pos {
                return Piece::White;
            }
        }
        for pos in self.black_pos.iter() {
            if given_pos == *pos {
                return Piece::Black;
            }
        }
        Piece::Empty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_position() {
        let p = Position::new(1,2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    #[should_panic]
    fn invalid_position() {
        let _ = Position::new(5, 0);
    }

    #[test]
    fn new_board() {
        let board = Board::new();
        let board = Board::from_u32(board.to_u32());
        for i in 0..4 {
            assert_eq!(board.get_piece(Position::new(0, i)), Piece::White);
            assert_eq!(board.get_piece(Position::new(4, i)), Piece::Black);
        }
    }
}
