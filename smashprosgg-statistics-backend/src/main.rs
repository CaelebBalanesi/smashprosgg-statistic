use std::error::Error;
mod game;
mod set;
mod connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = connection::get_all_sets().await;
    println!("{:#?}", data.unwrap().len());
    Ok(())
}