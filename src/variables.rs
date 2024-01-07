use crate::alphabet;
use crate::grid::Grid;

/// Where translation of problem data from/to integer variables occurs.
///
/// There are two kinds of variables:
///
/// - Cell variables: For each pair (cell,letter) is associated a variable. See
///   [Self::cell()] for the translation.
/// - Slot variables: For each pair (slot,word) is associated a variable. They are placed
///   "after" the cell variables in the model. See [Self::slot] for the translation.

/// The number of values that a cell of a solved grid can take.
pub const NUMBER_OF_CELL_VALUES: usize = alphabet::number_of_letters() + 1 /* block */;

/// The numerical representation of a block (the value of a shaded cell).
pub const BLOCK_INDEX: usize = alphabet::number_of_letters();

struct Variables<'solve_duration> {
    /// The crossword grid
    grid: &'solve_duration Grid,
    /// The number of words in the dictionary
    word_count: usize,
}

impl<'solve_duration> Variables<'solve_duration> {
    /// Creates a new instance.
    fn new(grid: &'solve_duration Grid, word_count: usize) -> Self {
        Variables { grid, word_count }
    }

    /// Returns the variable associated to the given at the given cell.
    ///
    /// Cell variables are put first in the model.
    ///
    /// <table>
    ///     <caption>Variable/letter association</caption>
    ///   <tr>
    ///     <th>Variable for (0,0)</th>
    ///     <td>1</td>
    ///     <td>2</td>
    ///     <td>3</td>
    ///     <td>...</td>
    ///     <td>26</td>
    ///     <td>27</td>
    ///     <th>Variable for (0,1)</th>
    ///     <td>28</td>
    ///     <td>29</td>
    ///     <td>30</td>
    ///     <td>...</td>
    ///     <td>53</td>
    ///     <td>54</td>
    ///     <th>etc.</th>
    ///   </tr>
    ///   <tr>
    ///     <th>Represented value</th>
    ///     <td>A</td>
    ///     <td>B</td>
    ///     <td>C</td>
    ///     <td>..</td>
    ///     <td>Z</td>
    ///     <td>#</td>
    ///     <th>Represented value</th>
    ///     <td>A</td>
    ///     <td>B</td>
    ///     <td>C</td>
    ///     <td>..</td>
    ///     <td>Z</td>
    ///     <td>#</td>
    ///     <th>etc.</th>
    ///   </tr>
    /// </table>
    fn cell(&self, row: usize, column: usize, value: usize) -> usize {
        row * self.grid.column_count() * NUMBER_OF_CELL_VALUES +
            column * NUMBER_OF_CELL_VALUES +
            value +
            1 // variable must be strictly positive
    }

    /// Returns the variable associated to the given word at the given slot.
    ///
    /// Slot variable are put after cell variables, so first slot variable corresponds to the number
    /// of cell variables (plus 1 because variables start at 1).
    fn slot(&self, slot_index: usize, word_index: usize) -> usize {
        self.cell_count() // last cell variable
            + slot_index * self.word_count
            + word_index
            + 1
    }

    /// Returns the number of cell variables.
    fn cell_count(&self) -> usize {
        self.grid.column_count() * self.grid.row_count() * NUMBER_OF_CELL_VALUES
    }

    /// Returns the number of slot variables.
    fn slot_count(&self) -> usize {
        self.grid.slot_count() * self.word_count
    }

    /// Returns the number of variables.
    fn count(&self) -> usize {
        self.cell_count() + self.slot_count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn variables_cell() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000 /* does not matter here */);

        assert_eq!(1, variables.cell(0, 0, 0));
        assert_eq!(2, variables.cell(0, 0, 1));
        assert_eq!(27, variables.cell(0, 0, 26));

        assert_eq!(28, variables.cell(0, 1, 0));
        assert_eq!(29, variables.cell(0, 1, 1));
        assert_eq!(54, variables.cell(0, 1, 26));

        assert_eq!(243, variables.cell(2, 2, 26))
    }

    #[test]
    fn variables_slot() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000);

        assert_eq!(244, variables.slot(0, 0));
        assert_eq!(245, variables.slot(0, 1));
        assert_eq!(100_243, variables.slot(0, 99_999));

        assert_eq!(100_244, variables.slot(1, 0));
        assert_eq!(100_245, variables.slot(1, 1));

        assert_eq!(600_243, variables.slot(5, 99_999));
    }

    #[test]
    fn variables_cell_count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000 /* does not matter here */);
        assert_eq!(243, variables.cell_count());
    }

    #[test]
    fn variables_slot_count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000);
        assert_eq!(600_000, variables.slot_count());
    }

    #[test]
    fn variables_count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(&grid, 100_000);
        assert_eq!(600_243, variables.count());
    }
}
