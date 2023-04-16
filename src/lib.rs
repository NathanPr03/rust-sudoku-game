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
mod game_handler;
mod trivia;
mod game_mode_service;
mod hint_service;
mod serialiser;
mod universe;
mod player;

pub use array_matrix::ArrayMatrix;
pub use fourbyfourcovermatrix::four_by_four_cover_matrix;
pub use iter::ColumnIterator;
pub use node::Node;
pub use node::StrongNode;
pub use node_matrix::NodeMatrix;
pub use solution_handler::find_solution;
pub use board_generator::BoardGenerator;
pub use util::pretty_print_board;
pub use util::check_if_move_is_valid;
pub use user_input::take_user_input_for_cell;
pub use user_input::get_trivia_input;
pub use user_input::get_save_game;
pub use user_input::get_users_start_game;
pub use user_input::get_game_mode;
pub use user_input_command::UserInputCommand;
pub use undo_handler::UndoHandler;
pub use game_handler::GameDifficulty;
pub use game_handler::GameHandler;
pub use trivia::Trivia;
pub use game_mode_service::determine_game_mode;
pub use hint_service::get_hint_command;
pub use serialiser::save;
pub use serialiser::load;
pub use universe::Universe;