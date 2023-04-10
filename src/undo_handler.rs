use std::collections::vec_deque::VecDeque;
use crate::{BOARD_SIZE, UserInputCommand};
use serde_derive::Serialize;
use serde_derive::Deserialize;
use colored::Colorize;

#[derive(Serialize, Deserialize)]
pub struct UndoHandler
{
    undo_stack: VecDeque<UserInputCommand>,
    redo_stack: VecDeque<UserInputCommand>
}

impl UndoHandler
{
    pub fn new() -> UndoHandler
    {
        return UndoHandler
        {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
        }
    }

    pub fn push_command(&mut self, command: UserInputCommand)
    {
        self.undo_stack.push_back(command);
    }

    pub fn undo_last_command
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.undo_stack.pop_back();
        if !command.is_some()
        {
            println!("{}", "There is no move to undo".red());
            return;
        }

        let unwrapped_command = command.unwrap();
        unwrapped_command.undo(sudoku_board);

        self.redo_stack.push_back(unwrapped_command);

        let success_message = "Move successfully undone".green();
        println!("{}", success_message);
    }

    pub fn redo_last_command
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.redo_stack.pop_back();
        if !command.is_some()
        {
            println!("{}", "There is no move to redo".red());
            return;
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(sudoku_board);

        self.undo_stack.push_back(unwrapped_command);

        let success_message = "Move successfully redone".green();
        println!("{}", success_message);
    }

    // Used for replays
    pub fn re_execute_all_commands
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.undo_stack.pop_back();
        if !command.is_some()
        {
            return;
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(sudoku_board);

        self.invalidate_redo_stack();
    }

    // Also used for replays
    pub fn redo_last_command_reverse
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.undo_stack.pop_front();
        if !command.is_some()
        {
            println!("{}", "No more moves were made, to continue playing interrupt the replay".red());
            return;
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(sudoku_board);

        self.redo_stack.push_back(unwrapped_command);

        let success_message = "Move successfully replayed".green();
        println!("{}", success_message);
    }

    pub fn undo_last_command_reverse
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.redo_stack.pop_back();
        if !command.is_some()
        {
            println!("{}", "There is no move to un-replay".red());
            return;
        }

        let unwrapped_command = command.unwrap();
        unwrapped_command.undo(sudoku_board);

        self.undo_stack.push_front(unwrapped_command);

        let success_message = "Move successfully un-replayed".green();
        println!("{}", success_message);
    }

    pub fn invalidate_redo_stack(&mut self)
    {
        self.redo_stack = VecDeque::new();
    }
}