pub struct ArrayMatrix {
    board_size: usize,
    board_size_squared: usize,
    exact_cover_matrix_columns: usize,
    exact_cover_matrix_rows: usize
}

impl ArrayMatrix {
    pub fn new(board_size: usize
    ) -> ArrayMatrix {
        return ArrayMatrix {
            board_size,
            board_size_squared: board_size * board_size,
            exact_cover_matrix_columns: board_size * 4,
            exact_cover_matrix_rows: board_size * board_size * board_size
        };
    }

    pub fn create_sparse_matrix(
        &mut self,
        cover_matrix: &mut Vec<Vec<usize>>,
        sudoku_board: &mut Vec<Vec<usize>>
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
        sudoku_board: &mut Vec<Vec<usize>>,
    ) {
        for row_index in 0..self.board_size {
            for column_index in 0..self.board_size {
                let value_of_cell = sudoku_board[row_index as usize][column_index as usize];
                if value_of_cell != 0 {
                    for i in 0..self.board_size {
                        if value_of_cell - 1 != i as usize {
                            let matrix_row_index =
                                (row_index * self.board_size_squared) + column_index * self.board_size + i;

                            for column in 0..self.exact_cover_matrix_columns {
                                cover_matrix[matrix_row_index as usize][column as usize] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn print_board(&self,
                       cover_matrix: &Vec<Vec<usize>>,
    ) -> () {
        println!("---------------------------------------------------------------------");
        for i in 0..cover_matrix.len() {
            for j in 0..cover_matrix[1].len() {
                if j % (self.board_size_squared) as usize == 0 {
                    print!("|");
                }
                print!("{}", cover_matrix[i][j]);
            }
            print!("|");
            println!();
            println!("---------------------------------------------------------------------")
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
        let mut pullback = self.board_size_squared;

        let mut column = self.board_size_squared;

        for row in 0..self.exact_cover_matrix_rows {
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
        let board_size_squared_times_two: u16 = ((self.board_size_squared) + (self.board_size_squared)) as u16;

        let mut column: u16 = board_size_squared_times_two;
        for row in 0..self.exact_cover_matrix_rows {
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
        let board_size_squared_times_three: usize =
            (self.board_size_squared) + (self.board_size_squared) + (self.board_size_squared);
        let mut pullback: usize = board_size_squared_times_three;
        let mut column: usize = board_size_squared_times_three;

        let sqrt_board_size: usize = (self.board_size as f32).sqrt() as usize;

        for row in 0..self.exact_cover_matrix_rows {
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
