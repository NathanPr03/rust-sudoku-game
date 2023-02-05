pub mod ninebyninecovermatrix;
pub mod fourbyfourcovermatrix;
mod node;
mod node_matrix;
mod array_matrix;
mod iter;

pub use node::Node;
pub use node::OwnedNode;
pub use node_matrix::NodeMatrix;
pub use array_matrix::ArrayMatrix;
pub use fourbyfourcovermatrix::four_by_four_cover_matrix;
pub use iter::ColumnIterator;

pub const BOARD_SIZE: u16 = 9;
pub const BOARD_SIZE_SQUARED: u16 = BOARD_SIZE * BOARD_SIZE;
pub const CONSTRAINTS: [&str; 4] = ["Position", "Row", "Column", "Square"];
pub const NUM_OF_CONSTRAINTS: u16 = CONSTRAINTS.len() as u16;
pub const EXACT_COVER_MATRIX_COLUMNS: u16 = BOARD_SIZE_SQUARED * NUM_OF_CONSTRAINTS;
pub const EXACT_COVER_MATRIX_ROWS: u16 = BOARD_SIZE_SQUARED * BOARD_SIZE;
pub const SOME_NEW: u16 = BOARD_SIZE;