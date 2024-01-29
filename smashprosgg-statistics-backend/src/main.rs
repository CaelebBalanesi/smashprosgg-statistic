use std::{fs, process::exit};
use serde_derive::Deserialize;

mod game;
mod set;
mod connection;
mod api;
mod statistics;
mod database;

#[derive(Deserialize)]
struct Config {
    api: Api,
}

#[derive(Deserialize)]
struct Api {
    ip: String,
}

fn main(){
    let filename = "Config.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
    let config: Config = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    api::start_api(config.api.ip);
}