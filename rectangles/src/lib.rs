use std::collections::VecDeque;

pub fn count(lines: &[&str]) -> usize {
    let field: Vec<Vec<Cell>> = lines.iter()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();

    if field.is_empty() || field[0].is_empty() {
        return 0;
    }

    let mut total = 0;

    for (y, row) in field.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &Cell::Intersection {
                // Fundamental strategy: find all intersections.
                // Attempt to trace a rectangle, assuming that this intersection
                // is the top-left corner. If that succeeds, add 1 to the output.
                // Complication: maybe it branches at an intersection?
                total += trace(&field, x, y);
            }
        }
    }
    total
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Intersection,
    Horizontal,
    Vertical,
    Blank,
}

impl From<char> for Cell {
    fn from(c: char) -> Cell {
        use Cell::*;
        match c {
            '+' => Intersection,
            '-' => Horizontal,
            '|' => Vertical,
            _ => Blank,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}

impl Direction {
    fn advance(self, x: usize, y: usize, x_bound: usize, y_bound: usize) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            Up if y > 0 => Some((x, y - 1)),
            Down if y < y_bound - 1 => Some((x, y + 1)),
            Left if x > 0 => Some((x - 1, y)),
            Right if x < x_bound - 1 => Some((x + 1, y)),
            _ => None,
        }
    }

    fn is_continuation(self, cell: Cell) -> bool {
        use Cell::*;
        use Direction::*;
        match (self, cell) {
            (_, Intersection) => true,
            (Up, Vertical) | (Down, Vertical) => true,
            (Right, Horizontal) |
            (Left, Horizontal) => true,
            (_, _) => false,
        }
    }

    fn next(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

/// Given a field of cells, and an initial x and y where there is an intersection,
/// trace the cells around and determine the number of rectangles which can be formed
/// using this intersection as a root.
fn trace(field: &Vec<Vec<Cell>>, initial_x: usize, initial_y: usize) -> usize {
    let advance = |direction: Direction, x, y| direction.advance(x, y, field[0].len(), field.len());

    let mut queue = VecDeque::with_capacity(1);
    // TTL initialized with 4: the number of corners in a rectangle
    queue.push_back((Direction::default(), initial_x, initial_y, 4));

    let mut output = 0;

    while queue.len() > 0 {
        let (direction, x, y, ttl) = queue.pop_front().unwrap();
        if let Some((next_x, next_y)) = advance(direction, x, y) {
            let next_cell = field[next_y][next_x];
            if (next_x, next_y) == (initial_x, initial_y) {
                output += 1;
            } else if next_cell == Cell::Intersection && ttl > 0 {
                // add both possible continuation directions to the queue
                // if we continue on in the same direction, we haven't bent, so
                // TTL remains the same. If we bend, though, we reduce TTL by 1.
                queue.push_back((direction, next_x, next_y, ttl));
                queue.push_back((direction.next(), next_x, next_y, ttl - 1));
            } else if direction.is_continuation(next_cell) && ttl > 0 {
                // simply continue moving in the same direction
                queue.push_back((direction, next_x, next_y, ttl));
            }
        }
    }

    output
}
