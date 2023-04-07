use crate::{BOARD_SIZE, UserInputCommand};
use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct UndoHandler
{
    pub undo_stack: Vec<UserInputCommand>,
    pub redo_stack: Vec<UserInputCommand>
}

impl UndoHandler
{
    pub fn new() -> UndoHandler
    {
        return UndoHandler
        {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn push_command(&mut self, command: UserInputCommand)
    {
        self.undo_stack.push(command);
    }

    pub fn undo_last_command
    (
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.undo_stack.pop();
        if !command.is_some()
        {
            println!("There is no move to undo");
            return;
        }

        let unwrapped_command = command.unwrap();
        unwrapped_command.undo(sudoku_board);

        self.redo_stack.push(unwrapped_command);
    }

    pub fn redo_last_command(
        &mut self,
        sudoku_board: &mut [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize]
    )
    {
        let command = self.redo_stack.pop();
        if !command.is_some()
        {
            println!("There is no move to redo");
            return;
        }

        let mut unwrapped_command = command.unwrap();
        unwrapped_command.execute(sudoku_board);

        self.undo_stack.push(unwrapped_command);
    }

    pub fn invalidate_redo_stack(&mut self)
    {
        self.redo_stack = Vec::new();
    }
}