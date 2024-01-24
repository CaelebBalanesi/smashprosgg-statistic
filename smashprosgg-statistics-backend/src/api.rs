use axum::{
    extract::Query, routing::get, Router
};
use crate::{connection, statistics};
use serde::{Serialize, Deserialize};

#[tokio::main]
pub async fn start_api() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hey my love <3" }))
        .route("/test", get(|| async { "Boom!" }))
        .route("/set_winrate", get(get_set_winrate))
        .route("/set_winrate/user", get(get_set_winrate_from_username))
        .route("/game_winrate", get(get_game_winrate))
        .route("/game_winrate/user", get(get_game_winrate_from_username));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2222").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct ThisIsTemp {
    user_id: usize,
}

async fn get_set_winrate(user_id: Query<ThisIsTemp>) -> String {
    println!("GET /set_winrate\nuser_id: {}", user_id.0.user_id);
    let sets = connection::get_all_sets(user_id.0.user_id).await;
    let win_rate = statistics::set_winrate(sets.unwrap(), user_id.0.user_id as u32);
    println!("Winrate: {:#?}", win_rate);
    win_rate.to_string()
}

async fn get_game_winrate(user_id: Query<ThisIsTemp>) -> String {
    println!("GET /game_winrate\nuser_id: {}", user_id.0.user_id);
    let sets = connection::get_all_sets(user_id.0.user_id).await;
    let win_rate = statistics::game_winrate(sets.unwrap(), user_id.0.user_id as u32);
    println!("Winrate: {:#?}", win_rate);
    win_rate.to_string()
}

#[derive(Serialize, Deserialize, Debug)]
struct Username {
    username: String,
}

async fn get_game_winrate_from_username(username: Query<Username>) -> String {
    println!("username: {}", username.0.username);
    let user_id = connection::get_user_id_from_username(username.0.username).await.unwrap();
    println!("GET /game_winrate/user\nuserId: {}", user_id);
    let sets = connection::get_all_sets(user_id).await;
    println!("{:#?}", sets);
    let win_rate = statistics::game_winrate(sets.unwrap(), user_id as u32);
    println!("Winrate: {:#?}", win_rate);
    win_rate.to_string()
}

async fn get_set_winrate_from_username(username: Query<Username>) -> String {
    println!("username: {}", username.0.username);
    let user_id = connection::get_user_id_from_username(username.0.username).await.unwrap();
    println!("GET /set_winrate/user\nuserId: {}", user_id);
    let sets = connection::get_all_sets(user_id).await;
    println!("{:#?}", sets);
    let win_rate = statistics::set_winrate(sets.unwrap(), user_id as u32);
    println!("Winrate: {:#?}", win_rate);
    win_rate.to_string()
}