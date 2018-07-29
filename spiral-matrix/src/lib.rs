const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn spiral_matrix(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; size]; size];
    if size == 0 {
        return matrix;
    }

    let mut direction_iter = DIRECTIONS.iter().cycle();
    let (mut drow, mut dcol) = direction_iter.next().unwrap();
    let mut row = 0;
    let mut col = 0;
    let mut count = 1;
    let mut just_continued = false;

    loop {
        matrix[row][col] = count;
        let (nrow, ncol) = (row as isize + drow, col as isize + dcol);
        if nrow < 0 || nrow as usize >= size || ncol < 0 || ncol as usize >= size
            || matrix[nrow as usize][ncol as usize] != 0
        {
            if just_continued {
                // two continues in a row means we must be done
                break;
            }
            // I'd prefer to express this as
            // &(drow, dcol) = direction_iter.next().unwrap();
            // but the compiler doesn't like that for some reason
            let next_directions = direction_iter.next().unwrap();
            drow = next_directions.0;
            dcol = next_directions.1;
            just_continued = true;
            continue;
        } else {
            just_continued = false;
        }

        row = nrow as usize;
        col = ncol as usize;
        count += 1;
    }

    matrix
}
