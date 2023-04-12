use serde_derive::Serialize;
use serde_derive::Deserialize;
use crate::GameDifficulty;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player
{
    name: String,
    moves_made: usize,
    hints_used: usize,
    undos_used: usize,
    redos_used: usize,
    trivias_answered: usize,
    pub game_difficulty: GameDifficulty
}

impl Player
{
    pub fn new(name: String, game_difficulty: GameDifficulty) -> Player
    {
        return Player
        {
            name,
            moves_made: 0,
            hints_used: 0,
            undos_used: 0,
            redos_used: 0,
            trivias_answered: 0,
            game_difficulty
        }
    }

    pub fn get_name(&self) -> String
    {
        return self.name.clone();
    }

    pub fn get_hints_used(&self) -> usize
    {
        self.hints_used
    }

    pub fn get_undos_used(&self) -> usize
    {
        self.undos_used
    }

    pub fn get_redos_used(&self) -> usize
    {
        self.redos_used
    }

    pub fn get_trivias_answered(&self) -> usize
    {
        self.trivias_answered
    }

    pub fn increment_moves_made(&mut self)
    {
        self.moves_made += 1;
    }

    pub fn increment_hints(&mut self)
    {
        self.hints_used += 1;
    }

    pub fn increment_undos(&mut self)
    {
        self.undos_used += 1;
    }

    pub fn increment_redos(&mut self)
    {
        self.redos_used += 1;
    }

    pub fn increment_trivias_answered(&mut self)
    {
        self.trivias_answered += 1;
    }
}

