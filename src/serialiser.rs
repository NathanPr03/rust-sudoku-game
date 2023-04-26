use crate::{GameHandler, get_save_game};
use std::fs::{create_dir, File, read_dir, read_to_string};
use std::io::Write;
use chrono::{DateTime, Local};

pub fn save(
    game_handler: &GameHandler
)
{
    let json = serde_json::to_string(&game_handler).unwrap();

    let now: DateTime<Local> = Local::now();
    let timestamp = now.to_rfc3339();

    let directory_path = format!("./src/savegames/{timestamp}");
    create_dir(directory_path.clone()).expect("Directory cannot be created, save game failed");

    let mut file = File::create(format!("{}/savegame.json", directory_path.clone())).unwrap();
    file.write_all(json.as_ref()).expect("Could not write undo stack");
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
        let datetime = DateTime::parse_from_rfc3339(unwrapped_dir.file_name().to_str().unwrap()).unwrap();
        let human_readable = datetime.format("%A, %B %d, %Y at %I:%M %p");

        let human_readable_as_string = human_readable.to_string().replace("\"", "");
        println!("{counter}: {}", human_readable_as_string);
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