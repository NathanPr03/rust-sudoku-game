pub struct ArrayMatrix {
    board_size: usize,
    board_size_squared: usize,
}

impl ArrayMatrix {
    pub fn new(
        board_size: usize
    ) -> ArrayMatrix {
        return ArrayMatrix {
            board_size,
            board_size_squared: board_size  * board_size
        };
    }

    pub fn create_sparse_matrix(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
        sudoku_board: &Vec<Vec<usize>>,
    ) -> () {
        self.generate_array_matrix(cover_matrix);

        return self.remove_clues_from_matrix(cover_matrix, sudoku_board);
    }

    fn generate_array_matrix(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
    ) -> () {
        self.cell_constraint(cover_matrix);
        self.row_constraint(cover_matrix);
        self.column_constraint(cover_matrix);
        self.region_constraint(cover_matrix);
    }

    fn remove_clues_from_matrix(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
        sudoku_board: &Vec<Vec<usize>>,
    ) {
        let exact_matrix_columns = cover_matrix[0].len();

        for row_index in 0..self.board_size {
            for column_index in 0..self.board_size {
                let value_of_cell = sudoku_board[row_index as usize][column_index as usize];
                if value_of_cell != 0 {
                    for i in 0..self.board_size {
                        if value_of_cell - 1 != i as usize {
                            let matrix_row_index =
                                (row_index * self.board_size_squared) + column_index * self.board_size + i;

                            for column in 0..exact_matrix_columns {
                                cover_matrix[matrix_row_index as usize][column as usize] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    fn cell_constraint(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
    ) -> () {
        let mut row_index = self.board_size;

        for column in 0..self.board_size_squared {
            for row in row_index - self.board_size..row_index {
                cover_matrix[row as usize][column as usize] = 1;
            }
            row_index = row_index + self.board_size;
        }
    }

    fn row_constraint(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
    ) -> () {
        let cover_matrix_rows = cover_matrix.len();
        let mut pullback = self.board_size_squared;

        let mut column  = self.board_size_squared;

        for row in 0..cover_matrix_rows {
            if row % self.board_size_squared == 0 && row > 1 {
                pullback += self.board_size
            }

            if column % self.board_size == 0 {
                column = pullback
            }

            cover_matrix[row as usize][column as usize] = 1;
            column += 1;
        }
    }

    fn column_constraint(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
    ) -> () {
        let cover_matrix_rows = cover_matrix.len();
        let board_size_squared_times_two = (self.board_size_squared) + (self.board_size_squared);

        let mut column = board_size_squared_times_two;
        for row in 0..cover_matrix_rows {
            if row % self.board_size_squared == 0 && row > 1 {
                column = board_size_squared_times_two;
            }

            cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }

    fn region_constraint(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
    ) -> () {
        let cover_matrix_rows = cover_matrix.len();

        let board_size_squared_times_three =
            (self.board_size_squared) + (self.board_size_squared) + (self.board_size_squared);
        let mut pullback = board_size_squared_times_three;
        let mut column = board_size_squared_times_three;

        let sqrt_board_size = (self.board_size as f32).sqrt() as usize;

        for row in 0..cover_matrix_rows {
            if row % (self.board_size_squared * sqrt_board_size) == 0 && row > 1 {
                pullback += self.board_size;
            } else if row % self.board_size_squared == 0 && row > 1 {
                pullback -= self.board_size * (sqrt_board_size - 1);
            } else if row % (self.board_size * sqrt_board_size) == 0 && row > 1 {
                pullback += self.board_size;
            }

            if column % self.board_size == 0 {
                column = pullback;
            }

            cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }
}
