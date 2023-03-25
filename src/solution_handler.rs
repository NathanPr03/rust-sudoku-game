use crate::nine_by_nine_cover_matrix::nine_by_nine_cover_matrix;
use crate::{ArrayMatrix, NodeMatrix, StrongNode, pretty_print_board, four_by_four_cover_matrix, sixteen_by_sixteen_cover_matrix};

pub fn find_solution(sudoku_board: &mut Vec<Vec<usize>>) -> bool {
    let board_size: usize  = sudoku_board.len();
    use std::time::Instant;
    let now = Instant::now();

    let mut is_solution_found = false;

    // Due to the way arrays work in rust its accessed cover_matrix[row_index][column_index]!!
    let mut cover_matrix: Vec<Vec<usize>> = nine_by_nine_cover_matrix();
    if board_size == 4 {
        cover_matrix = four_by_four_cover_matrix();
    } else if board_size == 16 {
        cover_matrix = sixteen_by_sixteen_cover_matrix();
    }

    let mut array_matrix = ArrayMatrix::new(board_size);
    array_matrix.create_sparse_matrix(&mut cover_matrix, sudoku_board);

    check_matrix_formed_properly(&cover_matrix);

    let mut nodes_matrix: NodeMatrix = NodeMatrix::new(board_size);
    nodes_matrix.arrange_matrix(&mut cover_matrix);
    nodes_matrix.search(0, now);

    if nodes_matrix.actual_solution.len() == 0
    {
        return is_solution_found;
    }else {
        println!("Solution length {}", nodes_matrix.actual_solution.len());
        is_solution_found = true;
    }

    convert_matrix_to_sudoku_grid(sudoku_board, nodes_matrix.actual_solution);

    let elapsed = now.elapsed();
    println!("Sudoku solved in: {:.2?}", elapsed);

    println!("BOARD FROM SOLUTION HANDLER");
    pretty_print_board(sudoku_board);
    return is_solution_found;
}

fn convert_matrix_to_sudoku_grid(
    sudoku_board: &mut Vec<Vec<usize>>,
    solution: Vec<StrongNode>,
) {
    let board_size: usize = sudoku_board.len();
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
    let exact_cover_matrix_columns = cover_matrix[0].len();
    for column in 0..exact_cover_matrix_columns {
        let mut one_found = false;
        for i in 0..cover_matrix.len() {
            if cover_matrix[i][column as usize] == 1 {
                one_found = true;
            }
        }

        if !one_found {
            panic!("No 1's in column: {}", column);
        }
    }
}
