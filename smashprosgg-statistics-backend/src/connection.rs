use std::error::Error;
use json::JsonValue;
use reqwest::get;
use crate::{game, set};


pub async fn get_all_sets() -> Result<Vec<set::Set>, Box<dyn Error>> {
    let resp = get("https://smashpros.gg/api/sets/user/15775/complete")
        .await?
        .text()
        .await?;

    let initial_data = json::parse(&resp).unwrap();
    let mut sets: Vec<set::Set> = vec![];

    let num_pages = initial_data["totalPages"].as_u16().unwrap();

    for i in 1..=num_pages {
        let data: JsonValue;
        if i == 1 {
            data = initial_data.clone();
        } else {
            let resp = get("https://smashpros.gg/api/sets/user/15775/complete")
                .await?
                .text()
                .await?;

            data = json::parse(&resp).unwrap();
        }

        for j in data["sets"].members() {
            let mut games: Vec<game::Game> = vec![];
            let game_data = get("https://smashpros.gg/api/sets/".to_string() + &j["id"].to_string())
                .await?
                .text()
                .await?;

            let game_data_parsed = json::parse(&game_data).unwrap();
            for k in game_data_parsed["games"].members() {
                let game: game::Game = game::Game {
                    id: k["id"].to_string(),
                    winnerId: k["winnerId"].as_u32().unwrap(),
                    stageId: k["stagePick"]["stageId"].as_u32().unwrap(),
                    date: k["createdAt"].to_string(),
                };
                games.push(game);
            }

            let set: set::Set = set::Set {
                id: j["id"].to_string(),
                score1: j["score1"].as_u8().unwrap(),
                score2: j["score2"].as_u8().unwrap(),
                user1Id: j["user1Id"].as_u32().unwrap(),
                user2Id: j["user2Id"].as_u32().unwrap(),
                winnerId: j["winnerId"].as_u32().unwrap(),
                date: j["createdAt"].to_string(),
                games: games,
            };
            sets.push(set);
        }
    }

    return Ok(sets);
}

