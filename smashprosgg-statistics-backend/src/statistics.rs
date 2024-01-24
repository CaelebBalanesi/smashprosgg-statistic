use crate::set::Set;

pub fn set_winrate(sets: Vec<Set>, user_id: u32) -> f32 {
    let mut wins: u16 = 0;
    let total: u16 = sets.len() as u16;

    for set in sets {
        if set.winner_id == user_id {
            wins += 1;
        }
    }

    println!("wins: {} total: {}", wins, total);

    return ( wins as f32 ) / ( total as f32 );
}

pub fn game_winrate(sets: Vec<Set>, user_id: u32) -> f32 {
    let mut wins: u16 = 0;
    let mut total: u16 = 0;
    for set in sets {
        for game in set.games {
            if game.winner_id == user_id {
                wins += 1;
            }
            total += 1;
        }
    }

    println!("wins: {} total: {}", wins, total);

    return ( wins as f32 ) / ( total as f32 );
}