use crate::game::Game;

#[derive(Debug)]
pub struct Set {
    pub id: String,
    pub score1: u8,
    pub score2: u8,
    pub user1Id: u32,
    pub user2Id: u32,
    pub winnerId: u32,
    pub date: String,
    pub games: Vec<Game>,
}