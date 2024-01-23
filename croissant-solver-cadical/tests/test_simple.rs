use croissant_crossword::crossword::{Crossword, CrosswordSolutions};
use croissant_solver_cadical::CadicalSolver;
use std::collections::HashSet;

// TODO share common methods with project splr/tests into a testkit project

#[test]
#[ignore = "fix me!"]
fn empty() {
    let solutions = solve("", []);
    assert_solutions_eq([], solutions);
}

#[test]
#[ignore = "fix me!"]
fn trivial() {
    let solutions = solve("...\n...\n...", ["AAA", "BBB", "CDE", "ABC", "ABD", "ABE"]);
    assert_solutions_eq(
        [
            "BBB\nBBB\nBBB",
            "ABC\nABD\nABE",
            "AAA\nBBB\nCDE",
            "AAA\nAAA\nAAA",
        ],
        solutions,
    );
}

#[test]
#[ignore = "fix me!"]
fn partially_prefilled_1x3() {
    let solutions = solve("AB.", ["ABC"]);
    assert_solutions_eq(["ABC"], solutions);
}

#[test]
#[ignore = "fix me!"]
fn partially_prefilled_3x3() {
    let solutions = solve("ABC\n...\n...", ["AAA", "BBB", "CDE", "ABC", "ABD", "ABE"]);
    assert_solutions_eq(["ABC\nABD\nABE"], solutions);
}

#[test]
#[ignore = "fix me!"]
fn with_blocks() {
    let solutions = solve("ABC\n..#\n#..", ["AA", "BBB", "ABC", "AB", "BE"]);
    assert_solutions_eq(["ABC\nAB#\n#BE"], solutions);
}

#[test]
fn impossible_no_solution() {
    let solutions = solve(
        "ABC\n...\n...",
        [
            "AAA", "BBB", "CDF", /* should be CDE */
            "ABC", "ABD", "ABE",
        ],
    );
    assert_solutions_eq([], solutions);
}

#[test]
fn impossible_no_candidate() {
    let solutions = solve("...\n...\n...", []);
    assert_solutions_eq([], solutions);
}

/// Solves the given grid using the cadical solver.
fn solve<const N: usize>(grid: &str, words: [&str; N]) -> CrosswordSolutions {
    let words_vec = words.iter().map(|&word| word.to_string()).collect();
    let crossword = Crossword::from(grid, &words_vec).unwrap();
    let solver = Box::new(CadicalSolver::new());
    crossword.solve_with(solver)
}

/// Helper to verify that all solutions are present, in any order.
fn assert_solutions_eq<const N: usize>(
    expected_solutions: [&str; N],
    mut actual_solutions: CrosswordSolutions,
) {
    let mut expected_solutions = HashSet::from(expected_solutions);
    while let Some(solution) = actual_solutions.next() {
        assert_eq!(
            true,
            expected_solutions.remove(solution.as_str()),
            "Unexpected solution: {solution:?}"
        );
    }
    assert_eq!(
        true,
        expected_solutions.is_empty(),
        "Missing solutions: {expected_solutions:?}"
    );
}
