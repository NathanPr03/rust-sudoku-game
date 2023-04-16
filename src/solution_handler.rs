use crate::ninebyninecovermatrix::nine_by_nine_cover_matrix;
use crate::{ArrayMatrix, NodeMatrix, StrongNode};

pub fn find_solution(sudoku_board: &mut Vec<Vec<usize>>)-> bool {
    use std::time::Instant;
    let now = Instant::now();

    let board_size = sudoku_board.len();
    let mut is_solution_found = false;

    // Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix: Vec<Vec<usize>> = nine_by_nine_cover_matrix();
    if board_size == 4 {
        // = four_by_four_cover_matrix();
    } else {
        // = nine_by_nine_cover_matrix();
    }

    let mut array_matrix = ArrayMatrix::new(9);
    array_matrix.create_sparse_matrix(&mut cover_matrix, &sudoku_board);
    // ArrayMatrix::print_board(&mut cover_matrix);

    check_matrix_formed_properly(&cover_matrix);

    let mut nodes_matrix: NodeMatrix = NodeMatrix::new();
    nodes_matrix.arrange_matrix(&cover_matrix);
    let cover_matrix_rows = cover_matrix.len();

    nodes_matrix.search(0, cover_matrix_rows);

    if nodes_matrix.actual_solution.len() == 0
    {
        return is_solution_found;
    }else {
        is_solution_found = true;
    }

    convert_matrix_to_sudoku_grid(sudoku_board, nodes_matrix.actual_solution);

    let elapsed = now.elapsed();
    println!("Sudoku solved in: {:.2?}", elapsed);

    return is_solution_found;
}

fn convert_matrix_to_sudoku_grid(
    sudoku_board: &mut Vec<Vec<usize>>,
    solution: Vec<StrongNode>,
) {
    let board_size = sudoku_board.len();
    let board_size_squared = board_size * board_size;

    for node in solution.clone() {
        let exact_cover_row_index = node.borrow().get_row().unwrap();
        let row = (exact_cover_row_index) / board_size_squared;
        let column = ((exact_cover_row_index) % board_size_squared) / board_size;
        let mut value = (exact_cover_row_index) % board_size;
        value = value + 1;
        if value == 0 {
            value = board_size;
        }

        sudoku_board[row][column] = value;
    }
}

fn check_matrix_formed_properly(
    cover_matrix: &Vec<Vec<usize>>,
) {
    let cover_matrix_columns = cover_matrix[0].len();

    for column in 0..cover_matrix_columns {
        let mut one_found = false;
        for i in 0..cover_matrix.len() {
            if cover_matrix[i][column] == 1 {
                one_found = true;
            }
        }

        if !one_found {
            panic!("No 1's in column: {}", column);
        }
    }
}
