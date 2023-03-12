use crate::{
    BOARD_SIZE, BOARD_SIZE_SQUARED, CONSTRAINTS, EXACT_COVER_MATRIX_COLUMNS,
    EXACT_COVER_MATRIX_ROWS, SQRT_BOARD_SIZE,
};

pub struct ArrayMatrix {
    cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize], //This should really take a reference
}

impl ArrayMatrix {
    pub fn new(
        cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
            EXACT_COVER_MATRIX_ROWS as usize],
    ) -> ArrayMatrix {
        return ArrayMatrix { cover_matrix };
    }

    pub fn get_cover_matrix(
        &self,
    ) -> &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize] {
        return &self.cover_matrix;
    }

    pub fn create_sparse_matrix(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
        sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    ) -> () {
        self.generate_array_matrix(cover_matrix);

        return self.remove_clues_from_matrix(cover_matrix, sudoku_board);
    }

    fn generate_array_matrix(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
        self.cell_constraint(cover_matrix);
        self.row_constraint(cover_matrix);
        self.column_constraint(cover_matrix);
        self.region_constraint(cover_matrix);
    }

    fn remove_clues_from_matrix(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
        sudoku_board: &[[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    ) {
        for row_index in 0..BOARD_SIZE {
            for column_index in 0..BOARD_SIZE {
                let value_of_cell = sudoku_board[row_index as usize][column_index as usize];
                if value_of_cell != 0 {
                    for i in 0..BOARD_SIZE {
                        if value_of_cell - 1 != i as usize {
                            let mut hack = 0;
                            if i == BOARD_SIZE {
                                hack = 1;
                            }
                            let matrix_row_index =
                                (row_index * BOARD_SIZE_SQUARED) + column_index * BOARD_SIZE + i;

                            for column in 0..EXACT_COVER_MATRIX_COLUMNS {
                                cover_matrix[matrix_row_index as usize][column as usize] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn print_board(
        cover_matrix: &[[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
             EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
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

    fn cell_constraint(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
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
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
        let mut pullback: u16 = BOARD_SIZE_SQUARED;

        let mut column: u16 = BOARD_SIZE_SQUARED;

        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % BOARD_SIZE_SQUARED == 0 && row > 1 {
                pullback += BOARD_SIZE
            }

            if column % BOARD_SIZE == 0 {
                column = pullback
            }

            cover_matrix[row as usize][column as usize] = 1;
            column += 1;
        }
    }

    fn column_constraint(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
        let board_size_squared_times_two: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);

        let mut column: u16 = board_size_squared_times_two;
        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % BOARD_SIZE_SQUARED == 0 && row > 1 {
                column = board_size_squared_times_two;
            }

            cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }

    fn region_constraint(
        &mut self,
        cover_matrix: &mut [[u32; EXACT_COVER_MATRIX_COLUMNS as usize];
                 EXACT_COVER_MATRIX_ROWS as usize],
    ) -> () {
        let board_size_squared_times_three: u16 =
            (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);
        let mut pullback: u16 = board_size_squared_times_three;
        let mut column: u16 = board_size_squared_times_three;

        let sqrt_board_size: u16 = (BOARD_SIZE as f32).sqrt() as u16;

        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % (BOARD_SIZE_SQUARED * sqrt_board_size) == 0 && row > 1 {
                pullback += BOARD_SIZE;
            } else if row % BOARD_SIZE_SQUARED == 0 && row > 1 {
                pullback -= BOARD_SIZE * (sqrt_board_size - 1);
            } else if row % (BOARD_SIZE * sqrt_board_size) == 0 && row > 1 {
                pullback += BOARD_SIZE;
            }

            if column % BOARD_SIZE == 0 {
                column = pullback;
            }

            cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }
}
