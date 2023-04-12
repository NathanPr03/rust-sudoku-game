use serde_derive::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player
{
    name: String,
    hints_used: usize,
    undos_used: usize,
    redos_used: usize,
}

impl Player
{
    pub fn new(name: String) -> Player
    {
        return Player
        {
            name,
            hints_used: 0,
            undos_used: 0,
            redos_used: 0,
        }
    }

    pub fn get_name(&self) -> String
    {
        return self.name.clone();
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
}

