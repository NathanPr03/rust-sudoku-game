use crate::{BOARD_SIZE, BOARD_SIZE_SQUARED, CONSTRAINTS, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS, SQRT_BOARD_SIZE};

pub struct ArrayMatrix {
    cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize] //This should really take a reference
}

impl ArrayMatrix {
    pub fn new(cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize])-> ArrayMatrix
    {
        return ArrayMatrix{
            cover_matrix
        }
    }

    pub fn get_cover_matrix(&self) -> &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
    {
        return &self.cover_matrix;
    }

    pub fn create_sparse_matrix
    (
        &mut self,
        sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
    ) -> ()
    {
        for row_index in 0..BOARD_SIZE {
            for column_index in 0..BOARD_SIZE {
                let value_of_cell = sudoku_board[row_index as usize][column_index as usize];
                if value_of_cell == 0 {
                    self.create_rows_for_non_clue
                    (
                        cover_matrix,
                        column_index,
                        row_index
                    );
                } else {
                    count += 1;
                    self.create_rows_for_clue
                    (
                        cover_matrix,
                        column_index,
                        row_index,
                        (value_of_cell - 1) as u16
                    );
                }
            }
        }
    }

    fn create_rows_for_non_clue
    (
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        column_index: u16,
        row_index: u16
    )
    {
        for value in 0..BOARD_SIZE {
            self.fill_in_array_matrix(cover_matrix, column_index, row_index, value);
        }
    }

    fn create_rows_for_clue
    (
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        column_index: u16,
        row_index: u16,
        value: u16
    )
    {
        self.fill_in_array_matrix(cover_matrix, column_index, row_index, value);
    }

    fn fill_in_array_matrix
    (
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        column_index: u16,
        row_index: u16,
        value: u16
    )
    {
        // These calculate which exact cover row the cell and value in the sudoku grid correspond to
        let matrix_row_index =
            column_index
                + (BOARD_SIZE * row_index)
                + (BOARD_SIZE_SQUARED * value);

        let x = column_index + (BOARD_SIZE * row_index) + (BOARD_SIZE_SQUARED * value);
        let x_min_column = (BOARD_SIZE * row_index) + (BOARD_SIZE_SQUARED * value);
        if matrix_row_index == 0 {
            let hi = 2;
        }
        let matrix_block_index =
            (column_index / SQRT_BOARD_SIZE)
                + ((row_index / SQRT_BOARD_SIZE)
                * SQRT_BOARD_SIZE);

        let matrix_column_index_row =
            SQRT_BOARD_SIZE * BOARD_SIZE
                * value
                + row_index as u16;

        let matrix_column_index_column =
            SQRT_BOARD_SIZE * BOARD_SIZE
                * value
                + BOARD_SIZE
                + column_index as u16;

        let matrix_column_index_block =
            SQRT_BOARD_SIZE * BOARD_SIZE
                * value
                + (CONSTRAINTS.len() / 2) as u16
                * BOARD_SIZE
                + matrix_block_index;

        let matrix_column_index_value =
            BOARD_SIZE_SQUARED
                +SQRT_BOARD_SIZE * BOARD_SIZE_SQUARED
                + (column_index as u16 + BOARD_SIZE * row_index as u16);

        cover_matrix[matrix_row_index as usize][matrix_column_index_row as usize] = 1;
        // cover_matrix[matrix_row_index as usize][matrix_column_index_column as usize] = 1;
        // cover_matrix[matrix_row_index as usize][matrix_column_index_block as usize] = 1;
        cover_matrix[matrix_row_index as usize][matrix_column_index_value as usize] = 1;

        self.generate_array_matrix(cover_matrix, matrix_row_index);
    }

    pub fn generate_array_matrix
    (
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        row_index: u16,
    )-> ()
    {
        // self.cell_constraint(cover_matrix);
        self.row_constraint(cover_matrix, row_index);
        // self.column_constraint(cover_matrix, row_index);
        self.region_constraint(cover_matrix, row_index);
    }

    pub fn print_board(
        cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
    ) -> ()
    {
        println!("---------------------------------------------------------------------");
        for i in 0..cover_matrix.len() {
            for j in 0..cover_matrix[1].len() {
                if j % (BOARD_SIZE_SQUARED) as usize == 0 {
                    print!("|");
                }
                print!("{}", cover_matrix[i][j]);
            }
            print!("|");
            println!();
            println!("---------------------------------------------------------------------")
        }
    }

    fn cell_constraint(&mut self, cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],) -> ()
    {
        let mut row_index: u16 = BOARD_SIZE;

        for column in 0..BOARD_SIZE_SQUARED {
            for row in row_index - BOARD_SIZE..row_index {
                cover_matrix[row as usize][column as usize] = 1;
            }
            row_index = row_index + BOARD_SIZE;
        }
    }

    fn row_constraint(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        row_index: u16,
    ) -> ()
    {
        let mut pullback: u16 = BOARD_SIZE_SQUARED;

        let mut column: u16 = BOARD_SIZE_SQUARED;

        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % BOARD_SIZE_SQUARED == 0 && row > 1 {
                pullback += BOARD_SIZE
            }

            if column % BOARD_SIZE == 0 {
                column = pullback
            }

            if row == row_index {
                cover_matrix[row as usize][column as usize] = 1;
            }
            column += 1;
        }
    }

    fn column_constraint(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        row_index: u16,
    ) -> ()
    {
        let board_size_squared_times_two: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);

        let mut column: u16 = board_size_squared_times_two;
        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % BOARD_SIZE_SQUARED == 0 && row > 1{
                column = board_size_squared_times_two;
            }
            if row == row_index {
                cover_matrix[row as usize][column as usize] = 1;
            }

            column += 1;
        }
    }

    fn region_constraint
    (
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize],
        row_index: u16,
    ) -> ()
    {
        let board_size_squared_times_three: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);
        let mut pullback: u16 = board_size_squared_times_three;
        let mut column: u16 = board_size_squared_times_three;

        let sqrt_board_size: u16 = (BOARD_SIZE as f32).sqrt() as u16;

        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % (BOARD_SIZE_SQUARED * sqrt_board_size) == 0 && row > 1
            {
                pullback += BOARD_SIZE;
            } else if row % BOARD_SIZE_SQUARED == 0 && row > 1
            {
                pullback -= BOARD_SIZE * (sqrt_board_size - 1);

            } else if row % (BOARD_SIZE * sqrt_board_size) == 0 && row > 1
            {
                pullback += BOARD_SIZE;
            }

            if column % BOARD_SIZE == 0 {
                column = pullback;
            }

            if row == row_index {
                cover_matrix[row as usize][column as usize] = 1;
            }

            column += 1;
        }
    }
}