use std::io::{stdin, stdout, Write};
use regex::Regex;
use crate::user_input_command::UserInputCommand;
use colored::Colorize;

pub fn get_single_players_name() -> String
{
    print!("Please enter your name: ");
    let users_name = get_user_input_generic();

    return users_name;
}

pub fn get_multiple_players_name(players: usize) -> Vec<String>
{
    let mut player_names = Vec::with_capacity(players);
    for i in 1..players + 1
    {
        print!("Please enter player {i}'s name: ");
        let player_name = get_user_input_generic();

        player_names.push(player_name);
    }

    return player_names;
}

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

        let error_message = format!("Invalid input, please select a number between 1 and {number_of_save_games}: ").red();
        println!("{error_message}");
    }
}

pub fn get_number_of_players() -> usize
{
    print!("Please enter the number of players you want (2-9): ");

    let mut num_of_players = 0;
    let mut valid = false;
    while !valid
    {
        let string_value = get_user_input_generic();

        let regex_string_two = format!(r"\b[2-9]\b");
        let valid_value = Regex::new(&*regex_string_two).unwrap();
        if !valid_value.is_match(&string_value)
        {
            let error_message = format!("Invalid value supplied, please make sure to enter a value between 2 and 9: ").red();
            print!("{error_message}");
        }else{
            num_of_players = string_value.parse::<i32>().unwrap() as usize;
            valid = true;
        }
    }

    return num_of_players;
}

pub fn get_game_mode() -> String
{
    print!("Please enter which game mode you would like to play:\n
     Very easy (ve): A 4x4 board, 60 minute timer; \n
     Easy (e): A 9x9 board with more clues, 30 minute timer; \n
     Medium (m): A 9x9 board with less clues, 20 minute timer; \n
     Hard (h): A 9x9 board with even less clues, 10 minute timer; \n
     Very hard (vh): A 16x6 board, 60 minute timer; \n
     Trivia (t): A 9x9 board, extra clues are added for every correct trivia answer, 20 minute timer:");

    loop
    {
        let user_input = get_user_input_generic();

        if user_input == "e" || user_input == "m" || user_input == "h" || user_input == "t" || user_input == "ve" || user_input == "vh" {
            return user_input;
        }
        let error_message = "Invalid input, please enter one of 've', 'e', 'm', 'h', 'vh', or 't': ".red();
        println!("{error_message}");
    }
}

pub fn get_trivia_input(question: String) -> String
{
    print!("Is the following trivia question true (t) or false (f): {question}: ");

    loop
    {
        let user_input = get_user_input_generic();

        if user_input == "t" || user_input == "f" {
            return user_input;
        }
        let error_message = "Invalid input, please enter either t or f: ".red();
        println!("{error_message}");
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
            let error_message = format!("Invalid coordinates supplied, please make sure to enter them in format: number,number. \
            Also make sure the number is between one and {board_size}").red();
            println!("{error_message}");
            valid = false;
            continue;
        }

        let mut iter = coordinates.splitn(2, ',');
        x = iter.next().unwrap().parse::<i32>().unwrap() as usize;
        y = iter.next().unwrap().parse::<i32>().unwrap() as usize;

        if x < 1 || x > board_size
        {
            let error_message = format!("{x} is not a valid x co-ordinate").red();
            println!("{error_message}");

            valid = false;
            continue;
        }

        if y < 1 || y > board_size
        {
            let error_message = format!("{y} is not a valid x co-ordinate").red();
            println!("{error_message}");

            valid = false;
            continue;
        }
    }

    return (x, y);
}

pub fn get_users_start_game() -> String
{
    print!("Please enter whether you would like to: \n
    Start a new game (n): A new single player game with a new randomly generated sudoku board; \n
    Load a previous game (l): Load a previous save game, the state of the board will be the same as when you saved it; \n
    Replay a previous game (r): Replay a previous save game, you will be able to step through every move you made, interrupting whenever you wish to make a move; \n
    Multiplayer game (m): A competitive multiplayer game where each player competes for score!: ");

    let users_move = get_user_input_generic();

    let viable_moves: [&str; 4] = ["n", "l", "r", "m"];

    if !viable_moves.contains(&&*users_move) {
        println!("{}", "Invalid move supplied. Please select one of (n), (l), (r), (m). Do not include the brackets in your input: ".red());
        return "Invalid".to_string();
    }

    return users_move;
}

pub fn get_users_replay_move() -> String
{
    print!("Please enter what move you would like to make: \
    continue replay (c), \
    undo last move (u), \
    interrupt replay and make a different move (i), \
    save game (s), \
    or quit (q): ");

    let users_move = get_user_input_generic();

    let viable_moves: [&str; 5] = ["c", "u", "i", "s", "q"];

    if !viable_moves.contains(&&*users_move) {
        println!("{}", "Invalid move supplied. Please select one of (c), (u), (i), (s), (q). Do not include the brackets in your input: ".red());
        return "Invalid".to_string();
    }

    return users_move;
}

pub fn get_users_two_player_move(player_name: String) -> String
{
    let mut valid = false;
    let mut users_move: String = "".to_string();
    while !valid {
        print!("{player_name} please enter what move you would like to make: \
            change a cell (c), \
            undo last move (u), \
            redo last move (r), \
            get a hint (h) \
            or pass (p): ");

        users_move = get_user_input_generic();

        let viable_moves: [&str; 5] = ["c", "u", "r", "h", "p"];

        if !viable_moves.contains(&&*users_move) {
            println!("{}", "Invalid move supplied. Please select one of (c), (u), (r), (h), (p). Do not include the brackets in your input: ".red());
        }else{
            valid = true;
        }
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
    save game (s), \
    or quit (q): ");

    let users_move = get_user_input_generic();

    let viable_moves: [&str; 6] = ["c", "u", "r", "h", "s", "q"];

    if !viable_moves.contains(&&*users_move) {
        println!("{}", "Invalid move supplied. Please select one of (c), (u), (r), (h), (s), (q). Do not include the brackets in your input: ".red());
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
        let error_message = format!("Invalid coordinates supplied, please make sure to enter them in format: number,number. \
        Also make sure the number is between one and {board_size}").red();
        println!("{error_message}");

        return None;
    }

    let mut iter = coordinates.splitn(2, ',');
    let x = iter.next().unwrap().parse::<i32>().unwrap() as usize;
    let y = iter.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut valid = true;
    if x < 1 || x > board_size
    {
        let error_message = format!("{x} is not a valid x co-ordinate").red();
        println!("{error_message}");

        valid = false
    }

    if y < 1 || y > board_size
    {
        let error_message = format!("{y} is not a valid y co-ordinate").red();
        println!("{error_message}");

        valid = false
    }

    if !valid {
        return None;
    }

    print!("Please enter the value you want to enter into the cell: ");

    let string_value = get_user_input_generic();

    let regex_string_two = format!(r"[1-9]");
    let valid_value = Regex::new(&*regex_string_two).unwrap();
    let int_value: usize = string_value.parse().unwrap();

    if !valid_value.is_match(&string_value) || int_value > board_size
    {
        let error_message = format!("Invalid value supplied, please make sure to enter a value between 1 and {board_size}").red();
        println!("{error_message}");
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