#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Mine,
    Num(usize),
}

impl Tile {
    fn as_char(self) -> char {
        use Tile::*;
        match self {
            Empty => ' ',
            Mine => '*',
            Num(n) => n.to_string().chars().last().unwrap(),
        }
    }

    fn increment(self) -> Self {
        use Tile::*;
        match self {
            Empty => Num(0),
            Mine => Mine,
            Num(n) => Num(n+1),
        }
    }
}

pub fn annotate(board: &[&str]) -> Vec<String> {
    let mut board: Vec<Vec<Tile>> = board.iter().map(|row| row.chars().map(|ch| {
        match ch {
            '*' => Tile::Mine,
            _ => Tile::Empty,
        }
    }).collect()).collect();

    let max_row = board.len();
    let max_col = if board.len() > 0 { board[0].len() } else { 0 };
    // always assuming a rectangular board, of course

    for row in 0..max_row {
        for col in 0..max_col {
            if board[row][col] == Tile::Mine {
                for (ar, ac) in rcs_surrounding(row, col, max_row, max_col) {
                    board[ar][ac] = board[ar][ac].increment();
                }
            }
        }
    }

    board.iter().map(|row| row.iter().map(|tile| tile.as_char()).collect()).collect()
}

/// Generate a list of (row, col) values surrounding a given (row, col).
/// Ensure output never has values < 0 or >= max_row
fn rcs_surrounding(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    if row > 0 {
        output.append(&mut gen_row(row - 1, col, max_col, false));
    }
    output.append(&mut gen_row(row, col, max_col, true));
    if row < max_row - 1 {
        output.append(&mut gen_row(row + 1, col, max_col, false));
    }
    output
}

/// Generate a list of (row, col) values in a specified row with (col - 1, col, col + 1).
/// Ensure output never has values < 0 or >= max_col
/// Optionally skip the center.
fn gen_row(row: usize, col: usize, max_col: usize, skip_center: bool) -> Vec<(usize, usize)> {
    let mut output = Vec::new();
    if col > 0 {
        output.push((row, col - 1));
    }
    if ! skip_center {
        output.push((row, col));
    }
    if col < max_col - 1 {
        output.push((row, col + 1));
    }
    output
}
