use std::thread::Builder;
use rust_sudoku_game::BOARD_SIZE;
use rust_sudoku_game::find_solution;

#[test]
pub fn test_9x9_sudoku_solved_correctly() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
        let mut board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        find_solution(&mut board);

        let result_board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        assert_eq!(&board, &result_board);
        })
        .unwrap();

    handler.join().unwrap();
}

#[test]
pub fn test_almost_empty_9x9_sudoku()
{
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {

        let mut board = [
            [9, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        find_solution(&mut board);

        let result_board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
            [9,1,2,3,4,5,6,7,8],
            [6,7,8,9,1,2,3,4,5],
            [3,4,5,6,7,8,9,1,2],
            [2,9,1,7,3,4,8,5,6],
            [5,8,6,2,9,1,7,3,4],
            [7,3,4,5,8,6,2,9,1],
            [1,2,9,4,6,3,5,8,7],
            [8,5,7,1,2,9,4,6,3],
            [4,6,3,8,5,7,1,2,9],
        ];

        assert_eq!(&board, &result_board);
    })
    .unwrap();

    handler.join().unwrap();
}

#[test]
pub fn test_almost_complete_9x9_sudoku()
{
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {

            let mut board = [
                [9, 1, 2, 3, 4, 5, 6, 7, 8],
                [6, 7, 8, 9, 1, 2, 3, 4, 5],
                [3, 4, 5, 6, 7, 8, 9, 1, 2],
                [2, 9, 1, 7, 3, 4, 8, 5, 6],
                [5, 8, 6, 2, 9, 1, 7, 3, 4],
                [7, 3, 4, 5, 8, 6, 2, 9, 1],
                [1, 2, 9, 4, 6, 3, 5, 8, 7],
                [8, 5, 7, 1, 2, 9, 4, 6, 3],
                [4, 6, 3, 8, 5, 7, 1, 2, 0],
            ];
            find_solution(&mut board);

            let result_board: [[usize; BOARD_SIZE as usize]; BOARD_SIZE as usize] = [
                [9,1,2,3,4,5,6,7,8],
                [6,7,8,9,1,2,3,4,5],
                [3,4,5,6,7,8,9,1,2],
                [2,9,1,7,3,4,8,5,6],
                [5,8,6,2,9,1,7,3,4],
                [7,3,4,5,8,6,2,9,1],
                [1,2,9,4,6,3,5,8,7],
                [8,5,7,1,2,9,4,6,3],
                [4,6,3,8,5,7,1,2,9],
            ];

            assert_eq!(&board, &result_board);
        })
        .unwrap();

    handler.join().unwrap();
}