extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    // Set the initial number of troops for each player.
    let mut player_troops = 20;
    let mut computer_troops = 20;

    // Start the game loop.
    while player_troops > 0 && computer_troops > 0 {
        // Ask the player how many dice they want to roll.
        println!("How many dice do you want to roll (1-3)?");
        let mut num_dice = String::new();
        io::stdin().read_line(&mut num_dice).expect("Failed to read line");
        let num_dice: u32 = match num_dice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Roll the dice for the player.
        let mut player_dice = Vec::new();
        for _ in 0..num_dice {
            player_dice.push(roll_dice());
        }
        println!("You rolled: {:?}", player_dice);

        // Roll the dice for the computer.
        let mut computer_dice = Vec::new();
        let num_computer_dice = 3 - num_dice;
        for _ in 0..num_computer_dice {
            computer_dice.push(roll_dice());
        }
        println!("The computer rolled: {:?}", computer_dice);

        // Compare the dice rolls and determine the winner.
        let mut player_wins = 0;
        let mut computer_wins = 0;
        for i in 0..num_dice {
            if player_dice[i] > computer_dice[i] {
                player_wins += 1;
            } else {
                computer_wins += 1;
            }
        }
        println!("You won {} rounds", player_wins);
        println!("The computer won {} rounds", computer_wins);

        // Update the number of troops for each player.
        player_troops -= player_wins;
        computer_troops -= computer_wins;
        println!("You have {} troops remaining", player_troops);
        println!("The computer has {} troops remaining", computer_troops);
    }

    // Determine the winner of the game.
    if player_troops > 0 {
        println!("Congratulations, you won the game!");
    } else {
        println!("Sorry, the computer won the game.");
    }
}

// Define a function to roll the dice.
fn roll_dice() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, 7)
}
