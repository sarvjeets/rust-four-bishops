use std::fmt;

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
        let pos: u8 = pos.try_into().unwrap();
        Position::new(pos / 4, pos % 4)
    }
}

enum Color {
    White,
    Black
}

struct Board {
    white_pos: [Position; 4],
    black_pos: [Position; 4],
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
                white_even_pos = white_even_pos << 4 | pos / 2;
            } else {
                white_odd_pos = white_odd_pos << 4 | pos / 2;
            }
        }

        for position in self.black_pos {
            let pos = position.to_u32();
            if pos % 2 == 0 {
                black_even_pos = black_even_pos << 4 | pos / 2;
            } else {
                black_odd_pos = black_odd_pos << 4 | pos / 2;
            }
        }

        white_even_pos << 24 | white_odd_pos << 16
        | black_even_pos << 8 | black_odd_pos
    }

    fn from_u32(encoded: u32) -> Self {
        let mask_8bit = 0x000000FFu32;
        let white_even_pos = encoded >> 24;
        let white_odd_pos = (encoded & mask_8bit << 16) >> 16;
        let black_even_pos = (encoded & mask_8bit << 8) >> 8;
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

    fn get_array(&self, color: Color) -> [bool; 20] {
        fn get_array(positions : &[Position; 4]) -> [bool; 20] {
            let mut ret = [false; 20];
            for i in positions.iter().map(|p| p.clone().to_u32() as usize) {
                ret[i] = true;
            }
            ret
        }
        match color {
            Color::White => return get_array(&self.white_pos),
            Color::Black => return get_array(&self.black_pos)
        }
    }

    // Returns all positions threatened by color pieces.
    fn threatened_array(&self, color: Color) -> [bool; 20] {
        let mut ret = [false; 20];

        let positions = match color {
            Color::White => &self.white_pos,
            Color::Black => &self.black_pos
        };

        fn inside(x: i16, y: i16) -> bool {
            x >= 0 && y >= 0 && x < 5 && y < 4
        }

        for pos in positions {
            let Position{x, y} = pos;
            let direction : [(i16, i16); 4] = [
                (-1, -1), (-1, 1), (1, -1), (1, 1)];
            for dir in direction.into_iter() {
                for i in 1..4 {
                    let x = *x as i16 + i * dir.0;
                    let y = *y as i16 + i * dir.1;

                    if !inside(x, y) {
                        break;
                    }

                    ret[Position::new(x as u8, y as u8).to_u32()
                        as usize] = true;
                }
            }
        }
        ret
    }

}

// Provide a printer for Board.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+---+---+")?;
        let white_array = self.get_array(Color::White);
        let black_array = self.get_array(Color::Black);

        for y in 0..4 {
            write!(f,  "\n|")?;
            for x in 0..5 {
                let index = Position::new(x, y).to_u32() as usize;
                let piece = if white_array[index] {
                    'W'
                } else if black_array[index] {
                    'B'
                } else {
                    ' '
                };
                write!(f, " {} |", piece)?;
            }
            write!(f, "\n+---+---+---+---+---+")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_position() {
        let p = Position::new(1,2);
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    #[should_panic]
    fn test_invalid_position() {
        let _ = Position::new(5, 0);
    }

    #[test]
    fn test_board_conversions() {
        let board = Board::from_u32(Board::new().to_u32());
        let white_array = board.get_array(Color::White);
        let black_array = board.get_array(Color::Black);
        for i in 0..4 {
            assert!(white_array[Position::new(0, i).to_u32() as usize]);
            assert!(black_array[Position::new(4, i).to_u32() as usize]);
        }
    }

    #[test]
    fn test_threatened() {
        let board = Board::new();
        let threatened_array = board.threatened_array(Color::White);
        println!("{:?}", threatened_array);
        for y in 0..3 {
            assert!(!threatened_array[Position::new(0, y).to_u32() as usize]);
            assert!(threatened_array[Position::new(1, y).to_u32() as usize]);
            assert!(threatened_array[Position::new(2, y).to_u32() as usize]);
            assert!(!threatened_array[Position::new(4, y).to_u32() as usize]);
        }
        assert!(threatened_array[Position::new(3, 0).to_u32() as usize]);
        assert!(!threatened_array[Position::new(3, 1).to_u32() as usize]);
        assert!(!threatened_array[Position::new(3, 2).to_u32() as usize]);
        assert!(threatened_array[Position::new(3, 3).to_u32() as usize]);
    }
}
