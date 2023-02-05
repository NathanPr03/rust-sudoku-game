use crate::{BOARD_SIZE, BOARD_SIZE_SQUARED, EXACT_COVER_MATRIX_COLUMNS, EXACT_COVER_MATRIX_ROWS};

pub struct ArrayMatrix {
    cover_matrix: [[u32; EXACT_COVER_MATRIX_COLUMNS as usize]; EXACT_COVER_MATRIX_ROWS as usize]
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

    pub fn generate_array_matrix(&mut self)-> ()
    {
        self.cell_constraint();
        self.row_constraint();
        self.column_constraint();
        self.region_constraint();
    }

    pub fn print_board(&mut self) -> ()
    {
        println!("---------------------------------------------------------------------");
        for i in 0..self.cover_matrix.len() {
            for j in 0..self.cover_matrix[1].len() {
                if j % (BOARD_SIZE_SQUARED) as usize == 0 {
                    print!("|");
                }
                print!("{}", self.cover_matrix[i][j]);
            }
            print!("|");
            println!();
            println!("---------------------------------------------------------------------")
        }
    }

    fn cell_constraint(&mut self) -> ()
    {
        let mut row_index: u16 = BOARD_SIZE;

        for column in 0..BOARD_SIZE_SQUARED {
            for row in row_index - BOARD_SIZE..row_index {
                self.cover_matrix[row as usize][column as usize] = 1;
            }
            row_index = row_index + BOARD_SIZE;
        }
    }

    fn row_constraint(&mut self) -> ()
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

            self.cover_matrix[row as usize][column as usize] = 1;
            column += 1;
        }
    }

    fn column_constraint(&mut self) -> ()
    {
        let board_size_squared_times_two: u16 = (BOARD_SIZE_SQUARED) + (BOARD_SIZE_SQUARED);

        let mut column: u16 = board_size_squared_times_two;
        for row in 0..EXACT_COVER_MATRIX_ROWS {
            if row % BOARD_SIZE_SQUARED == 0 && row > 1{
                column = board_size_squared_times_two;
            }
            self.cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }

    fn region_constraint(&mut self) -> ()
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

            self.cover_matrix[row as usize][column as usize] = 1;

            column += 1;
        }
    }
}