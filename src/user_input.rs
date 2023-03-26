use std::io::{stdin, stdout, Write};
use regex::Regex;

pub fn take_user_input_for_cell(board_size: usize)
{
    print!("Please enter a cell you want to change. For example 1,5 denotes row 1 column 5: ");

    let coordinates = take_user_input_generic();

    let regex_string = format!(r"^(?:[1-9]|{board_size}),(?:[1-9]|{board_size})$");
    let valid_coordinates_regex = Regex::new(&*regex_string).unwrap();
    if !valid_coordinates_regex.is_match(&coordinates)
    {
        println!("Invalid coordinates supplied, please make sure to enter them in format: number,number. \
        Also make sure the number is between one and {board_size}");
        return;
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
        return;
    }

    println!("{}", x);
    println!("{}", y);

    print!("Please enter the value you want to enter into the cell: ");

    let value = take_user_input_generic();

    let regex_string_two = format!(r"\b[1-{board_size}]\b");
    let valid_value = Regex::new(&*regex_string_two).unwrap();
    if !valid_value.is_match(&value)
    {
        println!("Invalid value supplied, please make sure to enter a value between 1 and {board_size}");
        return;
    }

    let integer_rep = value.parse::<i32>().unwrap() as usize;

    println!("{}", value);
}

fn take_user_input_generic() -> String
{
    let _=stdout().flush();
    let mut user_input = String::new();

    stdin().read_line(&mut user_input).expect("failed to readline");

    return user_input.to_string().replace("\n", "");
}