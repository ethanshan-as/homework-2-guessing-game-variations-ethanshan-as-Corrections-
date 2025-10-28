[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/VIVQWJOt)
# DS 210 Homework 2: Guessing Game Variations
README Requirements:

Replace this README.md with your own new file (rename this file if you want to preserve it).
Include an overview of your solution, documenting any assumptions or design decisions you made
Comment on the relative effectiveness of the random and optimal strategies. Is our midpoint strategy truly optimal? Do you think you could you do worse than the random strategy?
Add a brief reflection on what you learned while working through the assignment

While working on this assignment, I contemplated the differences between random and optimal strategies, and in the process, I implemented both approaches for an automated number-guessing game. This system enables the device to play the game independently by incorporating functions such as setting the questioner, updating the range, generating the secret number, and comparing strategies. Test results clearly show that the binary search strategy outperforms the random strategy. I attribute this to the binary search typically requiring approximately log₂ guesses, whereas the random strategy averages around 6-7 guesses. However, one strategy isn't always universally superior to the other, as different scenarios may favor different approaches.

Throughout the implementation of this assignment, I learned about Rust function composition, range handling, cargo build & test workflows, and how to manage boundary issues using saturating_add and saturating_sub safely. I also practiced Git branch management, as my previous GitHub experience was primarily limited to Kaggle practice problems. The main challenges encountered were resolving `unimplemented!()` errors and debugging test failures. Through iterative debugging and analysis of the command error logs, I gradually addressed these issues.

This was a highly engaging assignment! I did forget to commit incrementally, but I have additional saved records to demonstrate my development process.


| Name                                            | Points | Description                                                                                      |
|-------------------------------------------------|--------|--------------------------------------------------------------------------------------------------|
| `make_random_guess_bounds_and_diversity`        | 1      | Checks that make_random_guess stays in bounds and produces diverse values.                       |
| `make_good_guess_test_cases`                    | 1      | Ensures make_good_guess correctly returns the midpoint in several cases.                         |
| `update_range_test_cases`                       | 1      | Ensures update_range updates bounds correctly for higher, lower, correct, and invalid responses. |
| `generate_secret_number_bounds_and_diversity`   | 1      | Checks that generate_secret_number stays in bounds and produces diverse values.                  |
| `give_response_to_guess_test_cases`             | 1      | Ensures give_response_to_guess returns "higher", "lower", or "correct" appropriately.            |
| `play_game_with_good_guesser_num_rounds`        | 1      | Ensures the good guesser never takes more than 7 guesses on [1,100].                             |
| `play_game_with_random_guesser_num_rounds`      | 1      | Ensures the random guesser converges to an average between 7 and 8 guesses.                      |
| `run_strategy_comparisons_converges_reasonably` | 1      | Ensures run_strategy_comparisons gives plausible averages and shows good < random.               |
| `COMMIT_COUNT_1`                                | 1      | Ensures at least 2 commits.                                                                      |
| `COMMIT_COUNT_2`                                | 1      | Ensures at least 4 commits.                                                                      |
| `COMMIT_COUNT_3`                                | 1      | Ensures at least 6 commits.                                                                      |
| `COMMIT_COUNT_4`                                | 1      | Ensures at least 8 commits.                                                                      |
| `COMMIT_COUNT_5`                                | 1      | Ensures at least 10 commits.                                                                      |
