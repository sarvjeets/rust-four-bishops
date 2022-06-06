use four_bishops::Board;

fn main() {
    let start = Board::new();
    let end = Board::from_pos(start.black_pos.clone(),
                              start.white_pos.clone());
    four_bishops::bfs(start, end);
}
