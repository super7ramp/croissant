use croissant_solver::{ConfigurableSolver, Solver, SolverConfigurator};

/// Implementation of [ConfigurableSolver].
pub struct CadicalSolver {
    solver: cadical::Solver,
    last_solution: Vec<i32>,
    no_more_solution: bool,
}

impl Default for CadicalSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CadicalSolver {
    pub fn new() -> Self {
        let solver = cadical::Solver::default();
        CadicalSolver {
            solver,
            last_solution: Vec::new(),
            no_more_solution: false,
        }
    }
    fn refute_last_solution(&mut self) {
        if self.last_solution.is_empty() {
            return;
        }
        // FIXME that's not as trivial as that, same solutions are returned. I don't understand why it doesn't work
        //  nor why it "works" with logic-ng solver.
        //  Idea: Try to filter on positive literals of the relevant variables (i.e. crossword cell variables) which
        //  could be given as a hint in SolverBuilder.
        let not_last_solution: Vec<i32> = self.last_solution.iter().map(|lit| -lit).collect();
        self.solver.add_clause(not_last_solution);
    }
}

impl SolverConfigurator for CadicalSolver {
    fn add_clause(&mut self, literals: &[i32]) {
        self.solver.add_clause(literals.to_vec());
    }
}

impl Iterator for CadicalSolver {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.no_more_solution {
            return None;
        }
        self.refute_last_solution();
        if self.solver.solve() != Some(true) {
            self.no_more_solution = true;
            return None;
        }
        let variables_count = self.solver.max_variable() as usize;
        let mut model = Vec::with_capacity(variables_count);
        for variable in 1..(variables_count + 1) {
            let variable_state = self
                .solver
                .value(variable as i32)
                .map(|pos| if pos { 1 } else { -1 })
                .unwrap_or_default();
            model.push(variable_state);
        }
        self.last_solution.clone_from(&model);
        Some(model)
    }
}

impl Solver for CadicalSolver {}
impl ConfigurableSolver for CadicalSolver {}
