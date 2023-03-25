use std::thread::Builder;
use rust_sudoku_game::find_solution;

#[test]
pub fn test_9x9_sudoku_solved_correctly() {
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(64 * 1024 * 1024); // 64MB of stack space

    let handler = builder
        .spawn(|| {
        let mut board: Vec<Vec<usize>> = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        find_solution(&mut board);

        let result_board: Vec<Vec<usize>> = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
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

        let mut board = vec![
            vec![9, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        find_solution(&mut board);

        let result_board: Vec<Vec<usize>> = vec![
            vec![9,1,2,3,4,5,6,7,8],
            vec![6,7,8,9,1,2,3,4,5],
            vec![3,4,5,6,7,8,9,1,2],
            vec![2,9,1,7,3,4,8,5,6],
            vec![5,8,6,2,9,1,7,3,4],
            vec![7,3,4,5,8,6,2,9,1],
            vec![1,2,9,4,6,3,5,8,7],
            vec![8,5,7,1,2,9,4,6,3],
            vec![4,6,3,8,5,7,1,2,9],
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

            let mut board = vec![
                vec![9, 1, 2, 3, 4, 5, 6, 7, 8],
                vec![6, 7, 8, 9, 1, 2, 3, 4, 5],
                vec![3, 4, 5, 6, 7, 8, 9, 1, 2],
                vec![2, 9, 1, 7, 3, 4, 8, 5, 6],
                vec![5, 8, 6, 2, 9, 1, 7, 3, 4],
                vec![7, 3, 4, 5, 8, 6, 2, 9, 1],
                vec![1, 2, 9, 4, 6, 3, 5, 8, 7],
                vec![8, 5, 7, 1, 2, 9, 4, 6, 3],
                vec![4, 6, 3, 8, 5, 7, 1, 2, 0],
            ];
            find_solution(&mut board);

            let result_board: Vec<Vec<usize>> = vec![
                vec![9,1,2,3,4,5,6,7,8],
                vec![6,7,8,9,1,2,3,4,5],
                vec![3,4,5,6,7,8,9,1,2],
                vec![2,9,1,7,3,4,8,5,6],
                vec![5,8,6,2,9,1,7,3,4],
                vec![7,3,4,5,8,6,2,9,1],
                vec![1,2,9,4,6,3,5,8,7],
                vec![8,5,7,1,2,9,4,6,3],
                vec![4,6,3,8,5,7,1,2,9],
            ];

            assert_eq!(&board, &result_board);
        })
        .unwrap();

    handler.join().unwrap();
}