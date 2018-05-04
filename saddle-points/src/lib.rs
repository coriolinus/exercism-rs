pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut rv = Vec::new();
    if input.len() > 0 {
        let min_width = input.iter().map(|row| row.len()).min().unwrap();
        if min_width > 0 {
            // horizontal array of min values in a given column
            let col_min = (0..min_width)
                .map(|coln| input.iter().map(|row| row[coln]).min().unwrap())
                .collect::<Vec<_>>();
            // vertical array of max values in a given row
            let row_max = input
                .iter()
                .map(|row| row.iter().max().unwrap())
                .collect::<Vec<_>>();
            for row_idx in 0..input.len() {
                for col_idx in 0..min_width {
                    let item = input[row_idx][col_idx];
                    if item == *row_max[row_idx] && item == col_min[col_idx] {
                        rv.push((row_idx, col_idx));
                    }
                }
            }
        }
    }
    rv
}
