extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    // Entry point for the kernel module
    let mut game = SpeechTherapyGame::new();
    game.init_game();
    loop {
        game.run_turn();
    }
}

pub struct SpeechTherapyGame {
    player_name: String,
    score: u32,
    words_to_speak: Vec<String>,
    current_word_index: usize,
}

impl SpeechTherapyGame {
    pub fn new() -> Self {
        SpeechTherapyGame {
            player_name: String::from("Player"),
            score: 0,
            words_to_speak: vec![
                String::from("apple"),
                String::from("banana"),
                String::from("cherry"),
                String::from("date"),
                String::from("elderberry"),
            ],
            current_word_index: 0,
        }
    }

    pub fn init_game(&mut self) {
        // Initialize the game, e.g., set up player name, score, etc.
        println!("Welcome to Speech Therapy Game!");
        println!("Player Name: {}", self.player_name);
        self.score = 0;
        self.current_word_index = 0;
    }

    pub fn run_turn(&mut self) {
        // Run a single turn of the game
        if self.current_word_index < self.words_to_speak.len() {
            let current_word = &self.words_to_speak[self.current_word_index];
            println!("Please say the word: {}", current_word);
            // Simulate player input (in a real scenario, this would come from user input)
            let player_input = String::from(current_word); // Assume correct for simplicity
            if self.check_player_input(&player_input) {
                self.score += 1;
                println!("Correct! Your score is now: {}", self.score);
            } else {
                println!("Incorrect. Try again.");
            }
            self.current_word_index += 1;
        } else {
            println!("Game Over! Final Score: {}", self.score);
            // Reset or exit the game
        }
    }

    pub fn check_player_input(&self, input: &str) -> bool {
        // Check if the player's input is correct
        let current_word = &self.words_to_speak[self.current_word_index];
        input == current_word
    }

    pub fn get_score(&self) -> u32 {
        // Get the current score
        self.score
    }

    pub fn set_player_name(&mut self, name: String) {
        // Set a new player name
        self.player_name = name;
    }
}
