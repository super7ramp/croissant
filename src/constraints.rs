use crate::grid::Grid;
use crate::solver_spi::Solver;
use crate::variables::{Variables, BLOCK_INDEX, NUMBER_OF_CELL_VALUES};
use crate::{alphabet, grid};

///
/// Where crossword problem constraints are built.
///
/// The constraints are:
///
/// - Each cell must contain one and only one letter from the alphabet or a block. See
///   [add_one_letter_or_block_per_cell_clauses_to].
/// - Each slot must contain one and only one word from the input word list. This is the tricky
///   part, as there must be a correspondence between cell variables and slot variables. Basically,
///   each slot variable - i.e. a representation of a (slot,word) pair - is equivalent to a
///   conjunction (= and) of cell variables - i.e. (cell,letter) pairs. See
///   [add_one_word_per_slot_clauses_to]
/// - Prefilled cells must be kept as is. See [add_input_grid_constraints_are_satisfied_clauses_to].
///
/// Implementation note: Functions here add rules to the solver passed as parameter. Although having
/// just a factory of constraints, to be applied separately, would be nice, it does not scale in
/// terms of memory: There are too many literals and clauses. Hence, the choice to add the clauses
///
struct Constraints<'constraints_construction> {
    grid: &'constraints_construction Grid,
    variables: &'constraints_construction Variables<'constraints_construction>,
}

impl<'constraints_construction> Constraints<'constraints_construction> {
    /// Constructs a new instance.
    fn new(
        grid: &'constraints_construction Grid,
        variables: &'constraints_construction Variables<'constraints_construction>,
    ) -> Self {
        Constraints { grid, variables }
    }

    /// Adds the clauses ensuring that each cell must contain exactly one letter from the alphabet -
    /// or a block - to the given solver.
    fn add_one_letter_or_block_per_cell_clauses_to(&self, solver: &mut dyn Solver) {
        let mut literals_buffer: Vec<i32> = Vec::with_capacity(NUMBER_OF_CELL_VALUES);
        for row in 0..self.grid.row_count() {
            for column in 0..self.grid.column_count() {
                // TODO check for interruption
                for letter_index in 0..alphabet::letter_count() {
                    let letter_variable = self.variables.cell(row, column, letter_index) as i32;
                    literals_buffer.push(letter_variable)
                }
                let block_variable = self.variables.cell(row, column, BLOCK_INDEX) as i32;
                literals_buffer.push(block_variable);
                solver.add_exactly_one(&literals_buffer);
                literals_buffer.clear();
            }
        }
    }

    /// Adds the clauses ensuring that each prefilled letter/block must be preserved to the given
    /// solver.
    fn add_input_grid_constraints_are_satisfied_clauses_to(&self, solver: &mut dyn Solver) {
        let mut literals_buffer: Vec<i32> = Vec::with_capacity(1);
        for row in 0..self.grid.row_count() {
            for column in 0..self.grid.column_count() {
                // TODO check for interruption
                let prefilled_letter = self.grid.letter_at(row, column);
                let literal = match prefilled_letter {
                    grid::EMPTY => {
                        // Disallow solver to create a block
                        -(self.variables.cell(row, column, BLOCK_INDEX) as i32)
                    }
                    grid::BLOCK => self.variables.cell(row, column, BLOCK_INDEX) as i32,
                    _ => {
                        let letter_index = alphabet::index_of(prefilled_letter).unwrap();
                        self.variables.cell(row, column, letter_index) as i32
                    }
                };
                literals_buffer.push(literal);
                solver.add_clause(&literals_buffer);
                literals_buffer.clear();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    struct TestSolver {
        clauses: Vec<Vec<i32>>,
        exactly_one_clauses: Vec<Vec<i32>>,
        and_clauses: HashMap<i32, Vec<i32>>,
    }

    impl TestSolver {
        fn new() -> Self {
            TestSolver {
                clauses: vec![],
                exactly_one_clauses: vec![],
                and_clauses: HashMap::new(),
            }
        }
    }

    impl Solver for TestSolver {
        fn add_clause(&mut self, literals: &Vec<i32>) {
            let literals_copy = literals.to_vec();
            self.clauses.push(literals_copy)
        }

        fn add_exactly_one(&mut self, literals: &Vec<i32>) {
            let literals_copy = literals.to_vec();
            self.exactly_one_clauses.push(literals_copy)
        }

        fn add_and(&mut self, literal: i32, conjunction: &Vec<i32>) {
            let conjunction_copy = conjunction.to_vec();
            self.and_clauses.insert(literal, conjunction_copy);
        }
    }

    #[test]
    fn constraints_add_one_letter_or_block_per_cell_clauses_to() {
        let mut test_solver = TestSolver::new();
        let grid = Grid::from("...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000);
        let constraints = Constraints::new(&grid, &variables);

        constraints.add_one_letter_or_block_per_cell_clauses_to(&mut test_solver);

        assert_eq!(true, test_solver.clauses.is_empty(), "Unexpected clauses");
        let expected_exactly_one_clauses: Vec<Vec<i32>> = vec![
            // For each cell, exactly one value among the 27 possible
            vec![
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27,
            ],
            vec![
                28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
                49, 50, 51, 52, 53, 54,
            ],
            vec![
                55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
                76, 77, 78, 79, 80, 81,
            ],
            vec![
                82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101,
                102, 103, 104, 105, 106, 107, 108,
            ],
            vec![
                109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
                125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
            ],
            vec![
                136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151,
                152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162,
            ],
        ];
        assert_eq!(
            expected_exactly_one_clauses,
            test_solver.exactly_one_clauses
        );
    }

    #[test]
    fn add_input_grid_constraints_are_satisfied_clauses_to() {
        let mut test_solver = TestSolver::new();
        let grid = Grid::from("A#..#Z").unwrap();
        let variables = Variables::new(&grid, 100_000);
        let constraints = Constraints::new(&grid, &variables);

        constraints.add_input_grid_constraints_are_satisfied_clauses_to(&mut test_solver);

        let expected_clauses = vec![
            vec![1],
            vec![54],
            vec![-81],
            vec![-108],
            vec![135],
            vec![161],
        ];
        assert_eq!(expected_clauses, test_solver.clauses);
        assert_eq!(
            true,
            test_solver.exactly_one_clauses.is_empty(),
            "Unexpected clauses"
        );
    }
}
