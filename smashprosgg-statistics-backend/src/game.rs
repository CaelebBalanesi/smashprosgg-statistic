use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterPicks {
    pub user1: (u32, u32),
    pub user2: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub winner_id: u32,
    pub stage_id: u32,
    pub date: String,
    pub character_picks: CharacterPicks,
}