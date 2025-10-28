//! Put all code for individual functions here!

use rand::Rng;
use std::cmp::Ordering;

///  Given low and high (inclusive), randomly guesses a number
/// 
/// ##  Arguments
/// * low, high range (integers)
/// 
/// ## Returns
/// * random guess (integer)
/// 
/// ## Test Case
/// `cargo test make_random_guess_bounds_and_diversity`
/// notes: and need to pass: cargo test make_good_guess_test_cases
pub fn make_random_guess(low: i32, high: i32) -> i32 {
    if low > high {
        return low;
    }
    let mut rng = rand::rng();
    rng.random_range(low..=high)
}

/// Guess "intelligently" between low and high.
/// 
/// ##  Arguments
/// * low, high range (integers)
/// 
/// ## Returns
/// * guess (integer)
/// 
/// ## Test Case
/// `cargo test make_good_guess_test_cases``
pub fn make_good_guess(low: i32, high: i32) -> i32 {

    unimplemented!(); 

    /* TODO: Remove these comments
    return // TODO: Complete this line -- loop up binary search as a hint
    */
}

/// Update the guessing range of new possible values. 
/// 
/// Should return a tuple (new low, new high)
/// 
/// ## Arguments
/// * guess (integer) -- the last guess that was made
/// * response (string slice) -- the hint based on the guess: "lower", "higher", "correct"
///
/// ### Returns
/// * A tuple (new_low, new_high) each integer
/// 
/// ### Test Case
/// `cargo test update_range_test_cases`
pub fn update_range(guess: i32, response: &str, low: i32, high: i32) -> (i32, i32) {

    unimplemented!(); // TODO: Complete this line

    /* TODO: Remove these comments
    match response {
        "higher" => {
            let new_low = // TODO: Complete this line
            (new_low, high)
        }
        "lower" => {
            let new_high =   // TODO: Complete this line
            (low, new_high)
        }
        "correct" => {
            (low, high)
        }
        _ => {  // catch all
            (low, high)
        }
    }
    */
}

/// Randomly generate a secret number between the min and max (inclusive)
/// 
/// ### Arguments
/// * min, max (integers)
/// 
/// ### Return Values
/// * secret_number (integer)
/// 
/// ### Test Case
/// `cargo test generate_secret_number_bounds_and_diversity`
pub fn generate_secret_number(min: i32, max: i32) -> i32 {
    
    unimplemented!();    // TODO: Remove this line.

    /* TODO: Remove these comments
    return  // TODO: Complete this line.
    */
}

/// Based on guess return hint of "higher", "lower", or "correct"
/// 
/// ### Test Case
/// `cargo test give_response_to_guess_test_cases`
pub fn give_response_to_guess(guess: i32, secret_number: i32) -> String {
    
    unimplemented!();    // TODO: Remove this line

    /* TODO: Remove these comments
    return match guess.cmp(&secret_number) {
        Ordering::Less =>  // TODO: Complete this line
        Ordering::Greater => // TODO: Complete this line
        Ordering::Equal =>   // TODO: Complete this line
    }
    */
}

/// Play the full guessing game with the `make_good_guess` function
/// 
///  (generate number and run guessing strategy) 
/// 
/// ### Test Case
/// `cargo test play_game_with_good_guesser_num_rounds`
pub fn play_game_with_good_guesser(min: i32, max: i32) -> i32 {
    
    unimplemented!();  // TODO: Remove this line

    /* TODO: Remove these comments
    let secret_number = generate_secret_number(min, max);

    let mut guess: i32;
    let mut low = min;
    let mut high = max;
    let mut steps = 0;

    // TODO: Make a loop that counts the number iterations, makes a guess
    // using the `make_good_gues` function, gives a hint, checks if it is
    // correct and returns with the number of steps, otherwise updates the
    // range.

    */
}

/// Play the full guessing game with the `make_random_guess` function
/// 
/// (generate number and run guessing strategy)
/// 
/// ### Test
/// `cargo test play_game_with_random_guesser_num_rounds`
pub fn play_game_with_random_guesser(min: i32, max: i32) -> i32 {
    
    unimplemented!();  // TODO: Remove this line

    /* TODO: Remove these comments
    let secret_number = generate_secret_number(min, max);

    let mut guess: i32;
    let mut low = min;
    let mut high = max;
    let mut steps = 0;

    // TODO: Make a similar loop as the previous function but uses the
    // `make_random_guess` function.
    */
}

/// Compare the two guessing strategies over multiple games
/// 
/// ### Return
/// * (average of good guesser, average of random guesser)
pub fn run_strategy_comparisons(min: i32, max: i32, num_rounds: usize) -> (f64, f64) {
    
    unimplemented!();  // TODO: Remove this line

    /* TODO: Remove these comments
    let mut cumulative_good_steps = 0;
    let mut cumulative_random_steps = 0;

    for _ in 0..num_rounds {
        // TODO: call each guess and accumulate the number of steps
        
    }

    let avg_good = // TODO: Calculate the average
    let avg_random = // TODO: Calculate the average

    (avg_good, avg_random)
    */
    
    }

// !---------------------DO NOT REMOVE THIS LINE. THIS SETS UP THE `cargo test` TEST CASES ---------------------------
#[cfg(test)]
pub mod tests;
