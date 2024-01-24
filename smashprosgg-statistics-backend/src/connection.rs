use std::error::Error;
use reqwest::get;
use crate::{game, set};


pub async fn get_all_sets(user_id: usize) -> Result<Vec<set::Set>, Box<dyn Error>> {
    let mut api = 1;
    println!("API CALL {api}");
    api += 1;

    // Getting the initial data for totalPages
    let resp = get("https://smashpros.gg/api/sets/user/".to_owned() + &user_id.to_string() + "/complete")
        .await?
        .text()
        .await?;

    let initial_data = json::parse(&resp).unwrap();
    let mut sets: Vec<set::Set> = vec![];

    let num_pages = initial_data["totalPages"].as_u16().unwrap();
    println!("Total Pages: {}", num_pages);


    println!("Page: 1");
    for set in initial_data["sets"].members() {
        let mut games: Vec<game::Game> = vec![];

        println!("API CALL {api}");
        api += 1;
        let game_data = get("https://smashpros.gg/api/sets/".to_string() + &set["id"].to_string())
            .await?
            .text()
            .await?;

        let game_data_parsed = json::parse(&game_data).unwrap();
        for game in game_data_parsed["games"].members() {
            let game_data: game::Game = game::Game {
                id: game["id"].to_string(),
                winner_id: game["winnerId"].as_u32().unwrap(),
                stage_id: game["stagePick"]["stageId"].as_u32().unwrap(),
                date: game["createdAt"].to_string(),
            };
            games.push(game_data);
        }

        let set: set::Set = set::Set {
            id: set["id"].to_string(),
            score1: set["score1"].as_u8().unwrap(),
            score2: set["score2"].as_u8().unwrap(),
            user1_id: set["user1Id"].as_u32().unwrap(),
            user2_id: set["user2Id"].as_u32().unwrap(),
            winner_id: set["winnerId"].as_u32().unwrap(),
            date: set["createdAt"].to_string(),
            games: games,
        };
        sets.push(set);
    }

    if num_pages == 1 {
        return Ok(sets);
    }

    for i in 2..=num_pages {
        println!("Page {}", i);

        println!("API CALL {api}");
        api += 1;
        let resp = get("https://smashpros.gg/api/sets/user/".to_owned() + &user_id.to_string() + "/complete?page=" + &i.to_string())
            .await?
            .text()
            .await?;

        let data = json::parse(&resp).unwrap();

        for j in data["sets"].members() {
            let mut games: Vec<game::Game> = vec![];
            println!("API CALL {api}");
            api += 1;
            let game_data = get("https://smashpros.gg/api/sets/".to_string() + &j["id"].to_string())
                .await?
                .text()
                .await?;

            let game_data_parsed = json::parse(&game_data).unwrap();
            for k in game_data_parsed["games"].members() {
                let game: game::Game = game::Game {
                    id: k["id"].to_string(),
                    winner_id: k["winnerId"].as_u32().unwrap(),
                    stage_id: k["stagePick"]["stageId"].as_u32().unwrap(),
                    date: k["createdAt"].to_string(),
                };
                games.push(game);
            }

            let set: set::Set = set::Set {
                id: j["id"].to_string(),
                score1: j["score1"].as_u8().unwrap(),
                score2: j["score2"].as_u8().unwrap(),
                user1_id: j["user1Id"].as_u32().unwrap(),
                user2_id: j["user2Id"].as_u32().unwrap(),
                winner_id: j["winnerId"].as_u32().unwrap(),
                date: j["createdAt"].to_string(),
                games: games,
            };
            sets.push(set);
        }
    }

    Ok(sets)
}

pub async fn get_user_id_from_username(username: String) -> Result<usize, Box<dyn Error>> {
    let resp = get("https://smashpros.gg/api/users/".to_owned() + &username.to_string())
    .await?
    .text()
    .await?;

    let data = json::parse(&resp).unwrap();
    Ok(data["id"].as_usize().unwrap())
}