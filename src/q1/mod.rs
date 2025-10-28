use rand::Rng;
use std::cmp::Ordering;

/// Given low and high (inclusive), randomly guesses a number
pub fn make_random_guess(low: i32, high: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low..=high)
}

/// Given low and high (inclusive), returns the "good" (midpoint) guess
pub fn make_good_guess(low: i32, high: i32) -> i32 {
    low + (high - low) / 2
}

/// NOTE: Tests and main expect signature: update_range(guess, response, low, high)
/// Returns updated (low, high). Responses: "higher", "lower", "correct"
pub fn update_range(guess: i32, response: &str, low: i32, high: i32) -> (i32, i32) {
    match response {
        "higher" => {
            // secret > guess -> new low is guess + 1
            let new_low = guess.saturating_add(1).max(low);
            (new_low, high)
        }
        "lower" => {
            // secret < guess -> new high is guess - 1
            let new_high = guess.saturating_sub(1).min(high);
            (low, new_high)
        }
        "correct" => (guess, guess),
        _ => (low, high),
    }
}

/// Generate secret number uniformly in [min, max]
pub fn generate_secret_number(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// NOTE: Tests expect signature: give_response_to_guess(guess, secret)
/// Compare secret to guess and return "higher", "lower" or "correct"
/// If secret > guess -> return "higher" (i.e., guess should go higher)
pub fn give_response_to_guess(guess: i32, secret: i32) -> &'static str {
    match secret.cmp(&guess) {
        Ordering::Greater => "higher",
        Ordering::Less => "lower",
        Ordering::Equal => "correct",
    }
}

/// Play one game using the "good" midpoint strategy.
/// Returns number of guesses taken.
pub fn play_game_with_good_guesser(min: i32, max: i32) -> i32 {
    let secret = generate_secret_number(min, max);
    let mut low = min;
    let mut high = max;
    let mut steps = 0;

    loop {
        if low > high {
            break;
        }
        let guess = make_good_guess(low, high);
        steps += 1;
        // NOTE: call order matches tests: (guess, secret)
        let response = give_response_to_guess(guess, secret);
        if response == "correct" {
            break;
        }
        let (new_low, new_high) = update_range(guess, response, low, high);
        low = new_low;
        high = new_high;
    }

    steps
}

/// Play one game using the "random" strategy (random picks inside current range).
pub fn play_game_with_random_guesser(min: i32, max: i32) -> i32 {
    let secret = generate_secret_number(min, max);
    let mut low = min;
    let mut high = max;
    let mut steps = 0;

    loop {
        if low > high {
            break;
        }
        let guess = make_random_guess(low, high);
        steps += 1;
        let response = give_response_to_guess(guess, secret);
        if response == "correct" {
            break;
        }
        let (new_low, new_high) = update_range(guess, response, low, high);
        low = new_low;
        high = new_high;
    }

    steps
}

/// Compare strategies over many rounds. Returns (avg_good, avg_random).
pub fn run_strategy_comparisons(min: i32, max: i32, num_rounds: usize) -> (f64, f64) {
    if num_rounds == 0 {
        return (0.0, 0.0);
    }

    let mut cumulative_good: usize = 0;
    let mut cumulative_random: usize = 0;

    for _ in 0..num_rounds {
        let g = play_game_with_good_guesser(min, max) as usize;
        let r = play_game_with_random_guesser(min, max) as usize;
        cumulative_good += g;
        cumulative_random += r;
    }

    let avg_good = cumulative_good as f64 / num_rounds as f64;
    let avg_random = cumulative_random as f64 / num_rounds as f64;
    (avg_good, avg_random)
}

#[cfg(test)]
pub mod tests;
