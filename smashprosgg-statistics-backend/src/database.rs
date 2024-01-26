use rusqlite::{Connection, Result};

fn create_database() -> Connection{
    let conn = Connection::open("./database.db").unwrap();
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS Set (
            id TEXT PRIMARY KEY,
            score1 INTEGER NOT NULL,
            score2 INTEGER NOT NULL,
            user1id INTEGER NOT NULL,
            user2id INTEGER NOT NULL,
            winnerid INTEGER NOT NULL,
            date TEXT NOT NULL,
            games TEXT NOT NULL
        )",
        (),
    );
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS Game (
            id TEXT PRIMARY KEY,
            winner TEXT NOT NULL,
            stage TEXT NOT NULL,
            date TEXT NOT NULL,
        )",
        (),
    );
    conn
}

fn add_set() {
    let conn = create_database();
    
}

fn get_set() {

}

fn add_game() {

}

fn get_game() {

}