use rust_sudoku_game::{ColumnIterator, Node, NodeMatrix, OwnedNode};
use crate::complete_nine_by_nine_matrix::completed_nine_by_nine_cover_matrix;

mod complete_nine_by_nine_matrix;


#[test]
pub fn test_node_matrix_columns()
{
    let completed_cover_matrix = completed_nine_by_nine_cover_matrix();

    let mut nodes_matrix = NodeMatrix::new();

    nodes_matrix.arrange_matrix(&completed_cover_matrix);

    let columns: &Vec<OwnedNode> = nodes_matrix.get_column_nodes();

    for i in 0..columns.len() {
        let header_node: &OwnedNode = &columns[i];
        let column_iterator = ColumnIterator::new(header_node);

        let column_number = header_node.borrow_mut().column.unwrap();
        let mut last_row_number = 0;

        for node in column_iterator {
            let raw_node = node.upgrade().unwrap();

            let current_column = raw_node.borrow_mut().column.unwrap();
            let current_row_number = raw_node.borrow_mut().get_row().unwrap();

            if current_row_number != 0 {
                assert!(current_row_number > last_row_number);
            }
            assert_eq!(current_column, column_number);

            last_row_number = current_row_number;
        }
    }
}

#[test]
pub fn test_node_matrix_rows()
{
    let completed_cover_matrix = completed_nine_by_nine_cover_matrix();

    let mut nodes_matrix = NodeMatrix::new();

    nodes_matrix.arrange_matrix(&completed_cover_matrix);

    let rows: &Vec<Vec<OwnedNode>> = nodes_matrix.get_rows();

    for row_index in 0..rows.clone().len() {
        let row = &rows[row_index];

        let last_column_number = 0;

        for node in row {
            let raw_node = node.borrow_mut();

            let current_column_number = raw_node.column.unwrap();
            let current_row_number = raw_node.get_row().unwrap();

            if last_column_number != 0 {
                assert!(current_column_number > last_column_number);
            }

            assert_eq!(row_index, current_row_number)
        }
    }
}