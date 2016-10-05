pub struct PascalsTriangle {
    rows: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: row_count as usize }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut major_axis = Vec::with_capacity(self.rows);
        // initialize the first few rows here
        if self.rows >= 1 {
            major_axis.push(vec![1]);
        }
        if self.rows >= 2 {
            major_axis.push(vec![1, 1]);
        }
        // generate subsequent rows here
        for row_number in 2..self.rows {
            let mut minor_axis = Vec::with_capacity(row_number);
            minor_axis.push(1);

            for index in 1..row_number {
                minor_axis.push(major_axis[row_number - 1][index - 1] +
                                major_axis[row_number - 1][index])
            }

            minor_axis.push(1);

            major_axis.push(minor_axis);
        }

        major_axis
    }
}
