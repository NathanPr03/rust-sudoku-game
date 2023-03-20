use crate::complete_nine_by_nine_matrix::completed_nine_by_nine_cover_matrix;
use rust_sudoku_game::{ColumnIterator, NodeMatrix, StrongNode};

mod complete_nine_by_nine_matrix;

#[test]
pub fn test_node_matrix_columns() {
    let mut completed_cover_matrix = completed_nine_by_nine_cover_matrix();
    let board_size = 9;

    let mut nodes_matrix = NodeMatrix::new(board_size);

    nodes_matrix.arrange_matrix(&mut completed_cover_matrix);

    let columns: &Vec<StrongNode> = nodes_matrix.get_column_nodes();

    for i in 0..columns.len() {
        let header_node: &StrongNode = &columns[i];
        let column_iterator = ColumnIterator::new(header_node);

        let column_number = header_node.borrow_mut().column_index.unwrap();
        let mut last_row_number = 0;

        for node in column_iterator {
            let raw_node = node.upgrade().unwrap();

            let current_column = raw_node.borrow_mut().column_index.unwrap();
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
pub fn test_node_matrix_rows() {
    let mut completed_cover_matrix = completed_nine_by_nine_cover_matrix();
    let board_size = 9;

    let mut nodes_matrix = NodeMatrix::new(board_size);

    nodes_matrix.arrange_matrix(&mut completed_cover_matrix);

    let rows: &Vec<Vec<StrongNode>> = nodes_matrix.get_rows();

    for row_index in 0..rows.clone().len() {
        let row = &rows[row_index];

        for node in row {
            let raw_node = node.borrow_mut();

            let current_row_number = raw_node.get_row().unwrap();

            let current_node_col_number = raw_node.column_index.unwrap();

            let node_to_right = raw_node.right.upgrade().unwrap();

            let right_node_col_number = node_to_right.borrow_mut().column_index.unwrap();

            // Since the list is circular, node pointers can 'wrap around' the list and point to nodes before the current one.
            // Nodes should only point to nodes behind them if the node behind them is in the first constraint (= to a board size squared)
            if right_node_col_number < board_size*board_size as usize {
                assert!(current_node_col_number > right_node_col_number);
            } else {
                assert!(right_node_col_number > current_node_col_number);
            }

            assert_eq!(row_index, current_row_number);
        }
    }
}

#[test]
pub fn test_cover() {
    let mut completed_cover_matrix = completed_nine_by_nine_cover_matrix();
    let board_size = 9;

    let mut nodes_matrix = NodeMatrix::new(board_size);

    nodes_matrix.arrange_matrix(&mut completed_cover_matrix);

    let column_nodes = nodes_matrix.get_column_nodes();
    let first_column_node = &column_nodes[0].clone();

    // This gets the amount of nodes in a column, where one of the nodes shares a row with a node from the column we are covering
    let first_node_in_col = first_column_node.borrow_mut().down.clone();
    let node_in_same_row_of_first_node = first_node_in_col
        .upgrade()
        .unwrap()
        .borrow_mut()
        .right
        .clone();
    let column_for_node_in_same_row = node_in_same_row_of_first_node
        .upgrade()
        .unwrap()
        .borrow_mut()
        .header
        .clone();
    let before_cover_count_of_column_for_node_in_same_row = column_for_node_in_same_row
        .upgrade()
        .unwrap()
        .borrow_mut()
        .get_count();

    let index_of_first_column = first_column_node.borrow_mut().column_index.unwrap();
    NodeMatrix::cover(first_column_node);

    // This gets the right node of the node to the left, would have previously been 'first_column_node' but should now be the next node
    let index_of_first_column_after_cover = first_column_node
        .borrow_mut()
        .left
        .upgrade()
        .unwrap()
        .borrow_mut()
        .right
        .upgrade()
        .unwrap()
        .borrow_mut()
        .column_index
        .unwrap();
    let index_of_column_node_to_the_right = first_column_node
        .borrow_mut()
        .right
        .upgrade()
        .unwrap()
        .borrow_mut()
        .column_index
        .unwrap();

    // This gets the amount of nodes in a column, where one of the nodes shares a row with a node from the column we are covering
    let after_cover_count_of_column_for_node_in_same_row = column_for_node_in_same_row
        .upgrade()
        .unwrap()
        .borrow_mut()
        .get_count();

    assert!(
        before_cover_count_of_column_for_node_in_same_row
            > after_cover_count_of_column_for_node_in_same_row
    );

    // This asserts the column we have covered is no longer being pointed to
    assert!(index_of_first_column_after_cover > index_of_first_column);
    assert_eq!(
        index_of_column_node_to_the_right,
        index_of_first_column_after_cover
    );
}

#[test]
pub fn test_cover_removes_column_from_node_matrix() {
    let mut completed_cover_matrix = completed_nine_by_nine_cover_matrix();
    let board_size = 9;

    let mut nodes_matrix = NodeMatrix::new(board_size);

    nodes_matrix.arrange_matrix(&mut completed_cover_matrix);

    let column_nodes = nodes_matrix.get_column_nodes();
    let first_column_node = &column_nodes[0].clone();

    let root_node = nodes_matrix.root_node;
    let mut index_of_right_of_root = root_node
        .borrow_mut()
        .right
        .upgrade()
        .unwrap()
        .borrow_mut()
        .column_index
        .unwrap();

    assert_eq!(index_of_right_of_root, 0);
    NodeMatrix::cover(first_column_node);

    index_of_right_of_root = root_node
        .borrow_mut()
        .right
        .upgrade()
        .unwrap()
        .borrow_mut()
        .column_index
        .unwrap();
    assert_eq!(index_of_right_of_root, 1);
}