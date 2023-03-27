extern crate core;

mod array_matrix;
pub mod fourbyfourcovermatrix;
mod iter;
pub mod ninebyninecovermatrix;
mod node;
mod node_matrix;
mod solution_handler;
mod board_generator;
mod util;
mod user_input;
mod user_input_command;
mod undo_handler;

pub use array_matrix::ArrayMatrix;
pub use fourbyfourcovermatrix::four_by_four_cover_matrix;
pub use iter::ColumnIterator;
pub use node::Node;
pub use node::StrongNode;
pub use node_matrix::NodeMatrix;
pub use solution_handler::find_solution;
pub use board_generator::BoardGenerator;
pub use board_generator::GameDifficulty;
pub use util::pretty_print_board;
pub use util::check_if_move_is_valid;
pub use user_input::take_user_input_for_cell;
pub use user_input_command::UserInputCommand;
pub use undo_handler::UndoHandler;

pub const BOARD_SIZE: u16 = 9;
pub const BOARD_SIZE_SQUARED: u16 = BOARD_SIZE * BOARD_SIZE;
pub const CONSTRAINTS: [&str; 4] = ["Position", "Row", "Column", "Square"];
pub const NUM_OF_CONSTRAINTS: u16 = CONSTRAINTS.len() as u16;
pub const EXACT_COVER_MATRIX_COLUMNS: u16 = BOARD_SIZE_SQUARED * NUM_OF_CONSTRAINTS;
pub const EXACT_COVER_MATRIX_ROWS: u16 = BOARD_SIZE_SQUARED * BOARD_SIZE;
// pub const EXACT_COVER_MATRIX_COLUMNS: u16 = 7;
// pub const EXACT_COVER_MATRIX_ROWS: u16 = 6;
pub const SQRT_BOARD_SIZE: u16 = BOARD_SIZE / BOARD_SIZE;
