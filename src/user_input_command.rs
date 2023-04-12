use colored::Colorize;
use crate::{BOARD_SIZE, check_if_move_is_valid};
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UserInputCommand {
    x_coordinate: usize, // Column co-ordinate
    y_coordinate: usize, // Row co-ordinate
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
        if !check_if_move_is_valid(sudoku_board, self.get_target_cell_and_value()) {
            return;
        }

        let zero_index_offset = 1;
        self.previous_value = sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset];

        sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset] = self.new_value;

        let success_message = format!("Successfully edited coordinates {},{}", self.x_coordinate, self.y_coordinate).green();
        println!("{}", success_message);
    }

    pub fn undo(
        &self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        if !check_if_move_is_valid(sudoku_board, self.get_target_cell_and_undo_value())
        {
            return;
        }

        let zero_index_offset = 1;
        sudoku_board[self.y_coordinate - zero_index_offset][self.x_coordinate - zero_index_offset] = self.previous_value;

        let success_message = format!("Successfully edited coordinates {},{}", self.x_coordinate, self.y_coordinate).green();
        println!("{}", success_message);
    }

    pub fn get_x_coordinate(&self) -> usize {
        self.x_coordinate
    }

    pub fn get_y_coordinate(&self) -> usize {
        self.y_coordinate
    }

    pub fn get_new_value(&self) -> usize {
        self.new_value
    }

    pub fn get_target_cell_and_value(&self) -> (usize, usize, usize)
    {
        return (self.x_coordinate -1 , self.y_coordinate - 1, self.new_value);
    }

    pub fn get_target_cell_and_undo_value(&self) -> (usize, usize, usize)
    {
        return (self.x_coordinate - 1, self.y_coordinate - 1, self.previous_value);
    }
}