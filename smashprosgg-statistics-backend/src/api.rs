use axum::{
    extract::Query, http::HeaderMap, response::IntoResponse, routing::get, Json, Router
};
use crate::{connection, game::Game, statistics};
use serde::{Serialize, Deserialize};

// Utility function to create CORS headers
fn create_cors_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET,POST,OPTIONS,DELETE,PUT".parse().unwrap());
    headers
}

#[tokio::main]
pub async fn start_api(ip: String) {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Works" }))
        .route("/set_winrate", get(get_set_winrate))
        .route("/set_winrate/user", get(get_set_winrate_from_username))
        .route("/game_winrate", get(get_game_winrate))
        .route("/game_winrate/user", get(get_game_winrate_from_username))
        .route("/map_winrate_overtime/user", get(get_map_winrate_overtime));

    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct ThisIsTemp {
    user_id: usize,
}

async fn get_set_winrate(user_id: Query<ThisIsTemp>) -> String {
    let sets = connection::get_all_sets(user_id.0.user_id).await;
    let win_rate = statistics::set_winrate(sets.unwrap(), user_id.0.user_id as u32);
    win_rate.to_string()
}

async fn get_game_winrate(user_id: Query<ThisIsTemp>) -> String {
    let sets = connection::get_all_sets(user_id.0.user_id).await;
    let win_rate = statistics::game_winrate(sets.unwrap(), user_id.0.user_id as u32);
    win_rate.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct Username {
    username: String,
}

async fn get_game_winrate_from_username(username: Query<Username>) -> impl IntoResponse {
    let headers = create_cors_headers();

    let user_id = connection::get_user_id_from_username(username.0.username).await.unwrap();
    let sets = connection::get_all_sets(user_id).await;
    let win_rate = statistics::game_winrate(sets.unwrap(), user_id as u32);
    (headers, win_rate.to_string())
}

async fn get_set_winrate_from_username(username: Query<Username>) -> String {
    let user_id = connection::get_user_id_from_username(username.0.username).await.unwrap();
    let sets = connection::get_all_sets(user_id).await;
    let win_rate = statistics::set_winrate(sets.unwrap(), user_id as u32);
    win_rate.to_string()
}

#[derive(Deserialize, Serialize)]
struct GameMapData {
    map: u32,
    win: bool,
    date: String,
    user_character: u32,
    opponent_character: u32,
}

async fn get_map_winrate_overtime(username: Query<Username>) -> impl IntoResponse {
    let headers = create_cors_headers();
    let user_id = connection::get_user_id_from_username(username.0.username).await.unwrap();
    let sets = connection::get_all_sets(user_id).await.unwrap();

    let mut data: Vec<GameMapData> = vec![];

    for set in sets {
        for game in set.games {
            let data_entry: GameMapData;
            if game.character_picks.user1.0 == user_id as u32 {
                data_entry = GameMapData{
                    map: game.stage_id,
                    win: game.winner_id == user_id as u32,
                    date: iso8601::datetime(&game.date.to_string()).unwrap().date.to_string(),
                    user_character: game.character_picks.user1.1,
                    opponent_character: game.character_picks.user2.1,
                };
                data.push(data_entry);
            } else {
                data_entry = GameMapData{
                    map: game.stage_id,
                    win: game.winner_id == user_id as u32,
                    date: iso8601::datetime(&game.date.to_string()).unwrap().date.to_string(),
                    user_character: game.character_picks.user2.1,
                    opponent_character: game.character_picks.user1.1,
                };
                data.push(data_entry);
            }
        }
    }


    (headers, Json(data))
}