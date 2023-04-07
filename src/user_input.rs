use std::io::{stdin, stdout, Write};
use regex::Regex;
use crate::user_input_command::UserInputCommand;

pub fn get_save_game(number_of_save_games: usize) -> usize
{
    print!("Please enter the number of the save game which you want to load, 1..{number_of_save_games}: ");

    loop {
        let user_input = get_user_input_generic();

        let regex_string_two = format!(r"\b[1-{number_of_save_games}]\b");
        let valid_value = Regex::new(&*regex_string_two).unwrap();
        if valid_value.is_match(&user_input)
        {
            return user_input.parse::<i32>().unwrap() as usize;
        }

        print!("Invalid input, please select a number between 1 and {number_of_save_games}: ");
    }
}

pub fn get_game_mode() -> String
{
    print!("Please enter which game mode you would like to play, easy, (e), medium (m), hard (h) or trivia (t). \
    Alternatively if you want to load a previous game (l): ");

    loop
    {
        let user_input = get_user_input_generic();

        if user_input == "e" || user_input == "m" || user_input == "h" || user_input == "t" || user_input == "l" {
            return user_input;
        }
        print!("Invalid input, please enter one of 'e', 'm', 'h', 't' or 'l': ");
    }
}

pub fn get_trivia_input(question: String) -> String
{
    print!("Is the following trivia question true (T) or false (F): {question}: ");

    loop
    {
        let user_input = get_user_input_generic();

        if user_input == "T" || user_input == "F" {
            return user_input;
        }
        print!("Invalid input, please enter either T or F: ");
    }
}

pub fn get_coordinates_for_hint(board_size: usize) -> (usize, usize)
{
    let mut valid = false;
    let mut x = 0;
    let mut y = 0;
    while !valid
    {
        valid = true;
        print!("Please enter a cell you want a hint for. For example 1,5 denotes column 1 row 5: ");

        let coordinates = get_user_input_generic();

        let regex_string = format!(r"^(?:[1-9]|{board_size}),(?:[1-9]|{board_size})$");
        let valid_coordinates_regex = Regex::new(&*regex_string).unwrap();
        if !valid_coordinates_regex.is_match(&coordinates)
        {
            println!("Invalid coordinates supplied, please make sure to enter them in format: number,number. \
            Also make sure the number is between one and {board_size}");
            valid = false;
            continue;
        }

        let mut iter = coordinates.splitn(2, ',');
        x = iter.next().unwrap().parse::<i32>().unwrap() as usize;
        y = iter.next().unwrap().parse::<i32>().unwrap() as usize;

        if x < 1 || x > board_size
        {
            println!("{} is not a valid x co-ordinate", x);
            valid = false;
            continue;
        }

        if y < 1 || y > board_size
        {
            println!("{} is not a valid y co-ordinate", y);
            valid = false;
            continue;
        }
    }

    return (x, y);
}

pub fn get_users_replay_move() -> String
{
    print!("Please enter what move you would like to make: \
    change a cell (c), \
    undo last move (u), \
    redo last move (r), \
    get a hint (h) \
    save game (s) \
    quit (q): ");

    let users_move = get_user_input_generic();

    let viable_moves: [&str; 6] = ["c", "u", "r", "h", "s", "q"];

    if !viable_moves.contains(&&*users_move) {
        println!("Invalid move supplied. Please select one of (c), (u), (r), (h). Do not include the brackets in your input: ");
        return "Invalid".to_string();
    }

    return users_move;
}

pub fn get_users_move() -> String
{
    print!("Please enter what move you would like to make: \
    change a cell (c), \
    undo last move (u), \
    redo last move (r), \
    get a hint (h) \
    save game (s) \
    quit (q): ");

    let users_move = get_user_input_generic();

    let viable_moves: [&str; 6] = ["c", "u", "r", "h", "s", "q"];

    if !viable_moves.contains(&&*users_move) {
        println!("Invalid move supplied. Please select one of (c), (u), (r), (h). Do not include the brackets in your input: ");
        return "Invalid".to_string();
    }

    return users_move;
}

pub fn take_user_input_for_cell(board_size: usize) -> Option<UserInputCommand>
{
    print!("Please enter a cell you want to change. For example 1,5 denotes column 1 row 5: ");

    let coordinates = get_user_input_generic();

    let regex_string = format!(r"^(?:[1-9]|{board_size}),(?:[1-9]|{board_size})$");
    let valid_coordinates_regex = Regex::new(&*regex_string).unwrap();
    if !valid_coordinates_regex.is_match(&coordinates)
    {
        println!("Invalid coordinates supplied, please make sure to enter them in format: number,number. \
        Also make sure the number is between one and {board_size}");
        return None;
    }

    let mut iter = coordinates.splitn(2, ',');
    let x = iter.next().unwrap().parse::<i32>().unwrap() as usize;
    let y = iter.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut valid = true;
    if x < 1 || x > board_size
    {
        println!("{} is not a valid x co-ordinate", x);
        valid = false
    }

    if y < 1 || y > board_size
    {
        println!("{} is not a valid y co-ordinate", y);
        valid = false
    }

    if !valid {
        return None;
    }

    print!("Please enter the value you want to enter into the cell: ");

    let string_value = get_user_input_generic();

    let regex_string_two = format!(r"\b[1-{board_size}]\b");
    let valid_value = Regex::new(&*regex_string_two).unwrap();
    if !valid_value.is_match(&string_value)
    {
        println!("Invalid value supplied, please make sure to enter a value between 1 and {board_size}");
        return None;
    }

    let value = string_value.parse::<i32>().unwrap() as usize;

    return Some(UserInputCommand::new(x, y, value));
}

fn get_user_input_generic() -> String
{
    let _=stdout().flush();
    let mut user_input = String::new();

    stdin().read_line(&mut user_input).expect("failed to readline");

    return user_input.to_string().replace("\n", "");
}