//! HW02 -- Guessing Game Variations
//! See README.md for more details

// Initially we turn off compiler warnings for unused code and variables, etc.
// After you are code complete, we suggest commenting these next two lines.
#![allow(dead_code, unused_variables)]
#![allow(unused_imports)]

// Almost all of your coding will be done in `src/q1/mod.rs`.
// It's a way of splitting source code between multiple files.
// We'll cover this more later. 
// Declares that the q1 module exists
mod q1;

// Imports individual functions from the q1 module.
use crate::q1::{
    make_random_guess,
    make_good_guess,
    update_range,
    generate_secret_number, 
    give_response_to_guess,
    play_game_with_good_guesser,
    play_game_with_random_guesser,
    run_strategy_comparisons
};


fn main() {
    let min = 1;
    let max = 100;


    let mut guess: i32;

    // Subproblem 1 -- Guessing strategies
    guess = make_random_guess(min, max);
    println!("make_random_guess returned {guess}");

    guess = make_good_guess(min, max);
    println!("make_good_guess returned {guess}");

    // Subproblem 2 -- Update range
    let mut low = min;
    let mut high = max;
    let response = "higher";

    (low, high) = update_range(guess, response, low, high);
    println!("After guessing {guess}, hint was {response}, so new bounds are ({low},{high})");

    // Subproblem 3 -- Secret keeper
    let secret_number = generate_secret_number(low, high);
    println!("Secret number is {secret_number}");

    let response = give_response_to_guess(guess, secret_number);
    println!("For secret number {secret_number} and guess of {guess}, hint is {response}");

    // Subproblem 4: Game functions
    let num_guesses = play_game_with_good_guesser(min, max);
    println!("It took {num_guesses} guesses with the good guesser with (min, max) ({min}, {max})");

    let num_guesses = play_game_with_random_guesser(min, max);
    println!("It took {num_guesses} guesses with the random guesser with (min, max) ({min}, {max})");

    // Subproblem 5: Strategy comparisons
    let num_rounds = 10000; // you can adjust this
    println!("Comparing guessing strategies on range [{min}, {max}] with {num_rounds} rounds...");

    let (avg_good, avg_random) = run_strategy_comparisons(min, max, num_rounds);

    println!("Results:");
    println!("  Good guesser (binary search):   {:.3} average guesses", avg_good);
    println!("  Random guesser (uniform random draw):  {:.3} average guesses", avg_random);
}
