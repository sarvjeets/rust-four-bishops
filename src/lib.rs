use std::fmt;

#[derive(Clone, Debug)]
pub struct Position {
    x: u8,  // The x cordinate, 0 <= x < 5
    y: u8,  // The y coordinate 0 <= x < 4
}

enum Direction {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl Direction {
    fn get_directions() -> [Self; 4] {
        [Direction::UpperLeft, Direction::UpperRight,
         Direction::LowerLeft, Direction::LowerRight]
    }
}

impl Position {
    fn new(x: u8, y: u8) -> Position {
        assert!(x < 5);
        assert!(y < 4);
        Position {x, y}
    }

    fn as_usize(&self) -> usize {
        (4 * self.x + self.y) as usize
    }

    // Encodes position in 4 bits.
    fn as_compact(&self) -> (Color, u32) {
        let c = if (self.x + self.y) % 2 == 0 {
            Color::White
        } else {
            Color::Black
        };
        (c, (2 * self.x + self.y / 2) as u32)
    }

    fn from_compact(c: Color, val: u32) -> Position {
        let x = val / 2;
        let y = (val % 2) * 2 + (match c {
            Color::White => x % 2,
            Color::Black => (x + 1) % 2
        });
        Position::new(x as u8, y as u8)
    }
}

struct Moves {
    pos: Position,
    dir: Direction,
}

impl Iterator for Moves {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        let Position{x, y} = self.pos;
        let delta = match self.dir {
            Direction::UpperLeft => (-1, -1),
            Direction::UpperRight => (1, -1),
            Direction::LowerLeft => (-1, 1),
            Direction::LowerRight => (1, 1),
        };
        let x : i8 = x as i8 + delta.0;
        let y : i8 = y as i8 + delta.1;

        if x < 0 || x > 4 || y < 0 || y > 3 {
            None
        } else {
            self.pos.x = x as u8;
            self.pos.y = y as u8;
            Some(self.pos.clone())
        }
    }
}

enum Color {
    White,
    Black
}

impl Color {
    fn invert(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White
        }
    }
}

pub struct Board {
    pub white_pos: [Position; 4],
    pub black_pos: [Position; 4],
}

impl Board {
    // Creates a new Board with pieces in initial positions.
    pub fn new() -> Self {
        let y_pos : [u8; 4] = [0, 1, 2, 3];
        let white_pos : [Position; 4] = y_pos.into_iter().map(
            |y| Position::new(0, y)).collect::<Vec<Position>>()
            .try_into().unwrap();
        let black_pos : [Position; 4] = y_pos.into_iter().map(
            |y| Position::new(4, y)).collect::<Vec<Position>>()
            .try_into().unwrap();
        Board::from_pos(white_pos, black_pos)
    }

    pub fn from_pos(white_pos: [Position; 4],
                    black_pos: [Position; 4]) -> Self {
        Board{white_pos, black_pos}
    }

    fn as_u32(&self) -> u32 {
        let mut white_white_pos = 0u32;
        let mut white_black_pos = 0u32;
        let mut black_white_pos = 0u32;
        let mut black_black_pos = 0u32;

        fn update(val: &mut u32, new_val: u32) -> () {
            if *val == 0 {
                *val = new_val;
            } else if *val > new_val {
                *val = *val << 4 | new_val;
            } else {
                *val = new_val << 4 | *val;
            }
        }

        for position in &self.white_pos {
            let (c, val) = position.as_compact();
            match c {
                Color::White => update(&mut white_white_pos, val),
                Color::Black => update(&mut white_black_pos, val),
            };
        }

        for position in &self.black_pos {
            let (c, val) = position.as_compact();
            match c {
                Color::White => update(&mut black_white_pos, val),
                Color::Black => update(&mut black_black_pos, val),
            };
        }

        white_white_pos << 24 | white_black_pos << 16
        | black_white_pos << 8 | black_black_pos
    }

    fn from_u32(encoded: u32) -> Self {
        let mask_8bit = 0x000000FFu32;
        let white_white_pos = encoded >> 24;
        let white_black_pos = (encoded & mask_8bit << 16) >> 16;
        let black_white_pos = (encoded & mask_8bit << 8) >> 8;
        let black_black_pos = encoded & mask_8bit;

        let mask_4bit = 0x0000000Fu32;
        let white_pos = [
            Position::from_compact(Color::White, white_white_pos >> 4),
            Position::from_compact(Color::White, white_white_pos & mask_4bit),
            Position::from_compact(Color::Black, white_black_pos >> 4),
            Position::from_compact(Color::Black, white_black_pos & mask_4bit)];

        let black_pos = [
            Position::from_compact(Color::White, black_white_pos >> 4),
            Position::from_compact(Color::White, black_white_pos & mask_4bit),
            Position::from_compact(Color::Black, black_black_pos >> 4),
            Position::from_compact(Color::Black, black_black_pos & mask_4bit)];
        Board::from_pos(white_pos, black_pos)
    }

    fn get_array(&self, color: &Color) -> [bool; 20] {
        fn get_array(positions : &[Position; 4]) -> [bool; 20] {
            let mut ret = [false; 20];
            for i in positions.iter().map(|p| p.as_usize()) {
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
    fn threatened_array(&self, color: &Color) -> [bool; 20] {
        let mut ret = [false; 20];

        let positions = match color {
            Color::White => &self.white_pos,
            Color::Black => &self.black_pos
        };

        for pos in positions {
            for dir in Direction::get_directions() {
                let moves = Moves{pos : pos.clone(), dir};
                for valid_move in moves {
                    ret[valid_move.as_usize()] = true;
                }
            }
        }
        ret
    }

    fn next_boards(&self, to_move: &Color) -> Vec<Board> {
        let (my_color_pos, other_color_pos) = match to_move {
            Color::White => (&self.white_pos, &self.black_pos),
            Color::Black => (&self.black_pos, &self.white_pos),
        };

        let mut my_color_vec = Vec::new();
        for pos in my_color_pos.iter() {
            my_color_vec.push(pos.clone());
        }
        let my_color_pieces = self.get_array(&to_move);
        let threatened = self.threatened_array(&to_move.invert());

        let mut next_boards = Vec::new();

        for _ in 0..4 {
            let pos = my_color_vec.remove(0);
            for dir in Direction::get_directions() {
                let moves = Moves{pos : pos.clone(), dir};
                for valid_move in moves {
                    let new_index = valid_move.as_usize();
                    if my_color_pieces[new_index] {
                        break;
                    }
                    if threatened[new_index] {
                        continue;
                    }
                    let mut new_my_color_vec = my_color_vec.clone();
                    new_my_color_vec.push(valid_move);
                    next_boards.push(match to_move {
                        Color::White => Board::from_pos(
                            new_my_color_vec.try_into().unwrap(),
                            other_color_pos.clone()),
                        Color::Black => Board::from_pos(
                            other_color_pos.clone(),
                            new_my_color_vec.try_into().unwrap()),
                    });
                }
            }
            my_color_vec.push(pos);
        }
        next_boards
    }
}

// Provide a printer for Board.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+---+---+---+---+---+")?;
        let white_array = self.get_array(&Color::White);
        let black_array = self.get_array(&Color::Black);

        for y in 0..4 {
            write!(f,  "\n|")?;
            for x in 0..5 {
                let index = Position::new(x, y).as_usize();
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


pub fn bfs(start: Board, end : Board) -> () {
    use std::collections::HashMap;

    let end_u32 = end.as_u32();
    let mut next_nodes = vec![Board::new()];
    let mut visited_nodes = HashMap::new();
    visited_nodes.insert(start.as_u32(), 0u32);

    let mut to_move = Color::White;
    let mut num_moves = 0;

    while !next_nodes.is_empty() {
        let mut new_next_nodes = Vec::new();
        for board in next_nodes {
            //println!("Processing:\n{}", board);
            let board_u32 = board.as_u32();
            if board_u32 == end_u32 {
                // Found solution.
                println!("Solution found in {} moves.", num_moves);
                return ();
                let mut sol = Vec::new();
                let mut curr = &end_u32;
                while *curr != 0 {
                    sol.push(curr);
                    curr = visited_nodes.get(&curr).unwrap();
                }

                for s in sol.iter().rev() {
                    println!("{}", Board::from_u32(**s));
                }
            }

            for next_board in board.next_boards(&to_move) {
                let key = next_board.as_u32();
                if !visited_nodes.contains_key(&key) {
                    visited_nodes.insert(key, board_u32);
                    new_next_nodes.push(next_board);
                }
            }

        }
        num_moves += 1;
        to_move = to_move.invert();
        next_nodes = new_next_nodes;
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
        let board = Board::from_u32(Board::new().as_u32());
        let white_array = board.get_array(&Color::White);
        let black_array = board.get_array(&Color::Black);
        for i in 0..4 {
            assert!(white_array[Position::new(0, i).as_usize()]);
            assert!(black_array[Position::new(4, i).as_usize()]);
        }
    }

    #[test]
    fn test_threatened() {
        let board = Board::new();
        let threatened_array = board.threatened_array(&Color::White);
        for y in 0..3 {
            assert!(!threatened_array[Position::new(0, y).as_usize()]);
            assert!(threatened_array[Position::new(1, y).as_usize()]);
            assert!(threatened_array[Position::new(2, y).as_usize()]);
            assert!(!threatened_array[Position::new(4, y).as_usize()]);
        }
        assert!(threatened_array[Position::new(3, 0).as_usize()]);
        assert!(!threatened_array[Position::new(3, 1).as_usize()]);
        assert!(!threatened_array[Position::new(3, 2).as_usize()]);
        assert!(threatened_array[Position::new(3, 3).as_usize()]);
    }

    #[test]
    fn test_new_move() {
        let board = Board::new();
        let next_boards = board.next_boards(&Color::White);
        assert_eq!(next_boards.len(), 4);
    }

}
