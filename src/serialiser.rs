use crate::{BOARD_SIZE, GameHandler, get_save_game, UserInputCommand};
use std::fs::{create_dir, File, read_dir, read_to_string};
use std::io::Write;
use chrono::{DateTime, Local};
use chrono::Utc;

pub fn save(
    game_handler: &GameHandler
)
{
    let json = serde_json::to_string(&game_handler).unwrap();

    let now: DateTime<Local> = Local::now();
    let timestamp = now.to_rfc3339();

    let directory_path = format!("./savegames/{timestamp}");
    create_dir(directory_path.clone()).expect("Directory cannot be created, save game failed");

    let mut file = File::create(format!("{}/savegame.json", directory_path.clone())).unwrap();
    file.write_all(json.as_ref()).expect("Could not write undo stack");
}

pub fn load() -> GameHandler
{
    let directories = read_dir("./savegames").unwrap();

    let mut counter = 0;
    println!("Save games to choose from: ");
    for directory in directories
    {
        counter += 1;
        println!("{counter}: {}", directory.unwrap().path().display());
    }

    let save_game = get_save_game(counter);

    counter = 1;

    let mut directory_path = "".to_string();
    for directory in read_dir("./savegames").unwrap()
    {
        if counter == save_game {
            directory_path = directory.unwrap().path().to_str().unwrap().to_string() + "/savegame.json";
        }
        counter += 1;
    }

    let save_game_json = read_to_string(directory_path)
        .expect("Cannot read from savegame, load failed");

    let game_handler: GameHandler = serde_json::from_str(&*save_game_json).unwrap();

    return game_handler;
}