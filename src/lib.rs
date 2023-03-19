extern crate core;

mod array_matrix;
pub mod sixteen_by_sixteen_cover_matrix;
pub mod four_by_four_cover_matrix;
mod iter;
pub mod nine_by_nine_cover_matrix;
mod node;
mod node_matrix;
mod solution_handler;
mod board_generator;
mod util;

pub use array_matrix::ArrayMatrix;
pub use four_by_four_cover_matrix::four_by_four_cover_matrix;
pub use iter::ColumnIterator;
pub use node::Node;
pub use node::StrongNode;
pub use node_matrix::NodeMatrix;
pub use solution_handler::find_solution;
pub use board_generator::BoardGenerator;
pub use board_generator::GameDifficulty;
pub use util::pretty_print_board;
pub use sixteen_by_sixteen_cover_matrix::sixteen_by_sixteen_cover_matrix;

