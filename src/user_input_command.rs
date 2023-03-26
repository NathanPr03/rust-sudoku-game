use crate::BOARD_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct UserInputCommand {
    x_coordinate: usize,
    y_coordinate: usize,
    new_value: usize,
    previous_value: usize
}

impl UserInputCommand {
    pub fn new(x: usize, y: usize, value: usize) -> UserInputCommand
    {
        return UserInputCommand
        {
            x_coordinate: x,
            y_coordinate: y,
            new_value: value,
            previous_value: 0
        }
    }

    pub fn execute(
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let zero_index_offset = 1;
        self.previous_value = sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset];

        sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset] = self.new_value;
    }

    pub fn undo(
        &self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let zero_index_offset = 1;
        sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset] = self.previous_value;
    }
}