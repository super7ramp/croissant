use crate::alphabet;
use crate::grid::Grid;

/// The number of values that a cell of a solved grid can take.
pub const CELL_VALUE_COUNT: usize = alphabet::letter_count() + 1 /* block */;

/// The numerical representation of a block (the value of a shaded cell).
pub const BLOCK_INDEX: usize = alphabet::letter_count();

/// Where translation of problem data from/to integer variables occurs.
///
/// There are two kinds of variables:
///
/// - Cell variables: For each pair (cell,letter) is associated a variable. See
///   [Self::cell()] for the translation.
/// - Slot variables: For each pair (slot,word) is associated a variable. They are placed
///   "after" the cell variables in the model. See [Self::slot] for the translation.
#[derive(Clone)]
pub struct Variables {
    /// The crossword grid
    grid: Grid,
    /// The number of words in the dictionary
    word_count: usize,
}

impl Variables {
    /// Creates a new instance.
    pub fn new(grid: Grid, word_count: usize) -> Self {
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
    pub fn cell(&self, row: usize, column: usize, value: usize) -> usize {
        row * self.grid.column_count() * CELL_VALUE_COUNT + column * CELL_VALUE_COUNT + value + 1
        // variable must be strictly positive
    }

    /// Returns the variable associated to the given word at the given slot.
    ///
    /// Slot variable are put after cell variables, so first slot variable corresponds to the number
    /// of cell variables (plus 1 because variables start at 1).
    pub fn slot(&self, slot_index: usize, word_index: usize) -> usize {
        self.cell_count() // last cell variable
            + slot_index * self.word_count
            + word_index
            + 1
    }

    /// Translates a vector of the variables states back to a crossword grid.
    pub fn back_to_domain(&self, model: &Vec<i32>) -> String {
        let mut output_grid =
            String::with_capacity(self.grid.row_count() * self.grid.column_count());
        for row in 0..self.grid.row_count() {
            for column in 0..self.grid.column_count() {
                for value in 0..CELL_VALUE_COUNT {
                    let variable = self.cell(row, column, value) - 1;
                    if model[variable] > 0 {
                        let character = match value {
                            BLOCK_INDEX => char::from_u32(BLOCK_INDEX as u32).unwrap(),
                            _ => alphabet::letter_at(value),
                        };
                        output_grid.insert(row * self.grid.row_count() + column, character);
                    }
                }
            }
        }
        output_grid
    }

    /// Returns the number of cell variables.
    fn cell_count(&self) -> usize {
        self.grid.column_count() * self.grid.row_count() * CELL_VALUE_COUNT
    }

    /// Returns the number of slot variables.
    fn slot_count(&self) -> usize {
        self.grid.slot_count() * self.word_count
    }

    /// Returns the number of variables.
    pub fn count(&self) -> usize {
        self.cell_count() + self.slot_count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cell() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(grid, 100_000 /* does not matter here */);

        assert_eq!(1, variables.cell(0, 0, 0));
        assert_eq!(2, variables.cell(0, 0, 1));
        assert_eq!(27, variables.cell(0, 0, 26));

        assert_eq!(28, variables.cell(0, 1, 0));
        assert_eq!(29, variables.cell(0, 1, 1));
        assert_eq!(54, variables.cell(0, 1, 26));

        assert_eq!(243, variables.cell(2, 2, 26))
    }

    #[test]
    fn slot() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(grid, 100_000);

        assert_eq!(244, variables.slot(0, 0));
        assert_eq!(245, variables.slot(0, 1));
        assert_eq!(100_243, variables.slot(0, 99_999));

        assert_eq!(100_244, variables.slot(1, 0));
        assert_eq!(100_245, variables.slot(1, 1));

        assert_eq!(600_243, variables.slot(5, 99_999));
    }

    #[test]
    fn cell_count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(grid, 100_000 /* does not matter here */);
        assert_eq!(243, variables.cell_count());
    }

    #[test]
    fn slot_count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(grid, 100_000);
        assert_eq!(600_000, variables.slot_count());
    }

    #[test]
    fn count() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let variables = Variables::new(grid, 100_000);
        assert_eq!(600_243, variables.count());
    }

    #[test]
    fn back_to_domain() {
        let grid = Grid::from("...").unwrap();
        let variables = Variables::new(grid, 1);
        let mut model = vec![];
        for _cell in 0..3 {
            model.push(1); // state of variable 'A' for the current cell
            for _variable in 1..CELL_VALUE_COUNT {
                model.push(-1) // states of variable 'A' to '#' for the current cell
            }
        }

        let solved_grid = variables.back_to_domain(&model);

        assert_eq!("AAA", solved_grid);
    }
}
