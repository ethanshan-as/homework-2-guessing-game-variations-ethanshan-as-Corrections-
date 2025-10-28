use crate::q1::*;
use std::collections::HashSet;

/// Checks that make_random_guess stays in bounds and produces diverse values.
#[test]
fn make_random_guess_bounds_and_diversity() {
    let low = 1;
    let high = 100;
    let draws = 10000;

    let mut seen = HashSet::new();
    for _ in 0..draws {
        let g = make_random_guess(low, high);
        assert!(
            (low..=high).contains(&g),
            "make_random_guess produced out-of-bounds value: {}",
            g
        );
        seen.insert(g);
    }

    assert!(
        seen.len() >= 80,
        "Insufficient diversity from make_random_guess: saw {} distinct of 100",
        seen.len(),
    );
}

/// Ensures make_good_guess correctly returns the midpoint in several cases.
#[test]
fn make_good_guess_test_cases() {
    let guess_1_100 = make_good_guess(1, 100);
    // Both ways of breaking ties are equally valid
    assert!(guess_1_100 == 50 || guess_1_100 == 51);
    assert_eq!(make_good_guess(8, 10), 9);

    let guess_1_4 = make_good_guess(1, 4);
    assert_eq!(guess_1_4, 2);

    assert_eq!(make_good_guess(5, 5), 5);
    assert_eq!(make_good_guess(-2, 2), 0);
}

/// Ensures update_range updates bounds correctly for higher, lower, correct, and invalid responses.
#[test]
fn update_range_test_cases() {
    assert_eq!(update_range(50, "lower", 1, 100), (1, 49));
    assert_eq!(update_range(50, "higher", 1, 100), (51, 100));
}

/// Checks that generate_secret_number stays in bounds and produces diverse values.
#[test]
fn generate_secret_number_bounds_and_diversity() {
    let min = 1;
    let max = 100;
    let draws = 10000;

    let mut seen = HashSet::new();
    for _ in 0..draws {
        let s = generate_secret_number(min, max);
        assert!(
            (min..=max).contains(&s),
            "generate_secret_number produced out-of-bounds value: {}",
            s
        );
        seen.insert(s);
    }

    assert!(
        seen.len() >= 80,
        "Insufficient diversity from generate_secret_number: saw {} distinct of {} options in {} samples\nEnsure generate_secret_number chooses randomly",
        seen.len(),
        max,
        draws
    );
}

/// Ensures give_response_to_guess returns "higher", "lower", or "correct" appropriately.
#[test]
fn give_response_to_guess_test_cases() {
    assert_eq!(give_response_to_guess(15, 30), "higher");
    assert_eq!(give_response_to_guess(30, 15), "lower");
    assert_eq!(give_response_to_guess(22, 22), "correct");
}

/// Ensures the good guesser never takes more than 7 guesses on [1,100].
#[test]
fn play_game_with_good_guesser_num_rounds() {
    for _ in 0..200 {
        let steps = play_game_with_good_guesser(1, 100);
        assert!(steps <= 7, "good guesser took {steps} (>7) guesses");
    }
}

/// Ensures the random guesser converges to an average between 7 and 8 guesses.
#[test]
fn play_game_with_random_guesser_num_rounds() {
    let mut sum = 0;
    let num_rounds = 5000;
    for _ in 0..num_rounds {
        let steps = play_game_with_random_guesser(1, 100);
        sum += steps;
    }
    let average: f64 = sum as f64 / num_rounds as f64;
    assert!(
        average >= 7.0 && average <= 8.5,
        "random guesser did not take between 7 and 8.5 steps on average"
    );
}

/// Ensures run_strategy_comparisons gives plausible averages and shows good < random.
#[test]
fn run_strategy_comparisons_converges_reasonably() {
    let num_rounds = 5000;
    let (good_avg, rand_avg) = run_strategy_comparisons(1, 100, num_rounds);

    assert!(
        good_avg >= 5.0 && good_avg <= 7.0,
        "good_avg out of plausible range: {good_avg}"
    );
    assert!(rand_avg > good_avg, "random should be worse on average");
    
    assert!(
        rand_avg < 8.5,
        "random should converge to approximately 7.5"
    )
}
