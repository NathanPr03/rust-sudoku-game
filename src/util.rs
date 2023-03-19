pub fn pretty_print_board(sudoku_board: &Vec<Vec<usize>>) {
    //TODO: Probably remove this and use one defined in lib.rs
    let sqrt_board_size = ((sudoku_board.len() as f32).sqrt()) as usize;
    println!("-------------------------");
    for (i, row) in sudoku_board.iter().enumerate() {
        if i % sqrt_board_size == 0 && i != 0 {
            println!("|-----------------------|");
        }

        for (j, &num) in row.iter().enumerate() {
            if j % sqrt_board_size == 0 {
                print!("| ");
            }
            print!("{} ", num);
        }
        println!("|");
    }
    println!("-------------------------");
}