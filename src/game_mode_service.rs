use crate::GameDifficulty;
use crate::user_input::get_game_mode;

pub fn determine_game_mode() -> GameDifficulty
{
    let game_mode = get_game_mode();
    return match game_mode.as_str() {
        "ve" => GameDifficulty::VeryEasy,
        "e" => GameDifficulty::Easy,
        "m" => GameDifficulty::Medium,
        "h" => GameDifficulty::Hard,
        "vh" => GameDifficulty::VeryHard,
        "t" => GameDifficulty::Trivia,
        _ => GameDifficulty::Medium
    }
}