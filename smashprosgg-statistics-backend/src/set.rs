use crate::game::Game;

#[derive(Debug)]
pub struct Set {
    pub id: String,
    pub score1: u8,
    pub score2: u8,
    pub user1_id: u32,
    pub user2_id: u32,
    pub winner_id: u32,
    pub date: String,
    pub games: Vec<Game>,
}