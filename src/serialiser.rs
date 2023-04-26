use crate::{GameHandler, get_save_game};
use std::fs::{create_dir_all, File, read_dir, read_to_string};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local, NaiveDateTime};

// This file handles the loading and saving of games

pub fn save(game_handler: &GameHandler) {
    let json = serde_json::to_string(&game_handler).unwrap();

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let formatted_timestamp = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d_%H-%M-%S").to_string())
        .unwrap_or_default();

    let dir_path = format!("./src/savegames/{}", formatted_timestamp);
    create_dir_all(&dir_path).expect("Directory couldnt be created, savegame failed");

    let file_path = format!("{}/savegame.json", dir_path);
    let mut file = File::create(&file_path).unwrap();
    file.write_all(json.as_ref()).expect("Cannot write to json file, savegame failed");
}


pub fn load() -> GameHandler
{
    let directories = read_dir("./src/savegames").unwrap();

    let mut counter = 0;
    println!("Save games to choose from: ");
    for directory in directories
    {
        let unwrapped_dir = directory.unwrap();

        // TODO: Drunk rn but clean up these calls to `unwrapped_dir`
        if unwrapped_dir.file_name().to_str().unwrap().clone() == ".gitkeep"
        {
            continue;
        }

        counter += 1;
        let file_path = unwrapped_dir.file_name();
        let file_name = Path::new(file_path.to_str().unwrap()).file_name().unwrap().to_str().unwrap();

        let datetime = NaiveDateTime::parse_from_str(file_name, "%Y-%m-%d_%H-%M-%S").unwrap();
        let local_datetime: DateTime<Local> = DateTime::from_utc(datetime, *Local::now().offset());
        let formatted_datetime = local_datetime.format("%A, %B %e %Y at %I:%M %p");

        println!("{counter}: {}", formatted_datetime);

    }

    let mut save_game = get_save_game(counter);

    counter = 0;
    save_game -= 1; // Handle 0 indexing

    let mut directory_path = "".to_string();
    for directory in read_dir("./src/savegames").unwrap()
    {
        let unwrapped_dir = directory.unwrap();
        let file_name = unwrapped_dir.file_name();
        let file_name_two = file_name.to_str().unwrap();
        if file_name_two == ".gitkeep"
        {
            continue;
        }

        if counter == save_game {
            directory_path = unwrapped_dir.path().to_str().unwrap().to_string() + "/savegame.json";
            break;
        }

        counter += 1;
    }

    let save_game_json = read_to_string(directory_path)
        .expect("Cannot read from savegame, load failed");

    let game_handler: GameHandler = serde_json::from_str(&*save_game_json).unwrap();

    return game_handler;
}