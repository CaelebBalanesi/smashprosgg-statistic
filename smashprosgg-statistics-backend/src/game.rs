#[derive(Debug)]
pub struct CharacterPicks {
    pub user1: (u32, u32),
    pub user2: (u32, u32),
}

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub winner_id: u32,
    pub stage_id: u32,
    pub date: String,
    pub character_picks: CharacterPicks,
}