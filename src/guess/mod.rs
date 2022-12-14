// Include code from guess module
mod game;

// Main guess game function
pub fn play_game() {
    println!("\nWelcome to Guess Game!\nNumbers range from 0 to 255.\n");

    // Create a mutable hl struct instance so that I can update the high low game state
    let mut hl = game::start_game();

    // Continue looping until the game is finished
    loop {
        if let game::RoundResult::Lose = hl.play_game_round() {
            break;
        }
    }
}