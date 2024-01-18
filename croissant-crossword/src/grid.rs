use crate::slot::Slot;
use crate::{alphabet, slot};

/// The character representing a block, i.e. a shaded cell.
pub const BLOCK: char = '#';

/// The character representing an empty cell.
pub const EMPTY: char = '.';

/// A crossword grid.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    rows: Vec<String>,
}

impl Grid {
    /// Attempts to create a new [Grid] from given rows. Function returns the grid if given input is valid, otherwise
    /// it returns an error containing details about the validation failure.
    fn new(rows: Vec<String>) -> Result<Self, String> {
        let validation_result = Grid::validate(rows);
        if validation_result.is_err() {
            return Err(validation_result.unwrap_err());
        }
        let grid = Grid {
            rows: validation_result.unwrap(),
        };
        Ok(grid)
    }

    /// Validates the given rows. Function returns the input rows if they are valid, otherwise it returns an error
    /// containing details about the validation failure.
    fn validate(rows: Vec<String>) -> Result<Vec<String>, String> {
        if rows.is_empty() {
            // Trivial case, empty grid is valid
            return Ok(rows);
        }
        let first_row_length = rows[0].len();
        for row_index in 0..rows.len() {
            let row = &rows[row_index];
            let row_length = row.len();
            if row_length != first_row_length {
                return Err(format!("Inconsistent number of columns: Row #{row_index} has {row_length} columns but row #0 has {first_row_length}"));
            }
            for value in row.chars() {
                if value != EMPTY && value != BLOCK && !alphabet::contains(value) {
                    return Err(format!("Invalid value at row #{row_index}: {value}"));
                }
            }
        }
        Ok(rows)
    }

    /// Attempts to build a [Grid] from the given string. Function returns the grid if given input is valid, otherwise
    /// it returns an error containing details about the validation failure.
    pub fn from(value: &str) -> Result<Self, String> {
        let rows: Vec<String> = value.split('\n').map(String::from).collect();
        Grid::new(rows)
    }

    /// Returns the letter at given position.
    /// Special character `#` is returned if the cell contains a block.
    /// Special character `.` is returned if the cell contains no value.
    pub fn letter_at(&self, row: usize, column: usize) -> char {
        self.rows[row].chars().nth(column).unwrap()
    }

    /// Returns the slots of this grid.
    pub fn slots(&self) -> Vec<Slot> {
        let mut slots = vec![];
        slots.append(self.across_slots().as_mut());
        slots.append(self.down_slots(slots.len()).as_mut());
        slots
    }

    /// Computes the across slots.
    fn across_slots(&self) -> Vec<Slot> {
        let mut slots = vec![];
        let row_count = self.row_count();
        let column_count = self.column_count();
        for row in 0..row_count {
            let mut column_start = 0;
            for column in column_start..column_count {
                if self.letter_at(row, column) == BLOCK {
                    if column - column_start >= slot::MIN_LEN {
                        slots.push(Slot::across(slots.len(), column_start, column, row));
                    }
                    column_start = column + 1;
                }
            }
            if column_count - column_start >= slot::MIN_LEN {
                slots.push(Slot::across(slots.len(), column_start, column_count, row));
            }
        }
        slots
    }

    /// Computes the down slots.
    fn down_slots(&self, start_index: usize) -> Vec<Slot> {
        let mut slots = vec![];
        let row_count = self.row_count();
        let column_count = self.column_count();
        for column in 0..column_count {
            let mut row_start = 0;
            for row in row_start..row_count {
                if self.letter_at(row, column) == BLOCK {
                    if row - row_start >= slot::MIN_LEN {
                        slots.push(Slot::down(
                            start_index + slots.len(),
                            row_start,
                            row,
                            column,
                        ));
                    }
                    row_start = row + 1;
                }
            }
            if row_count - row_start >= slot::MIN_LEN {
                slots.push(Slot::down(
                    start_index + slots.len(),
                    row_start,
                    row_count,
                    column,
                ));
            }
        }
        slots
    }

    /// Returns the number of columns of the grid.
    pub fn column_count(&self) -> usize {
        if self.rows.is_empty() {
            0
        } else {
            self.rows[0].len()
        }
    }

    /// Returns the number of rows of the grid.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns the number of slots.
    pub fn slot_count(&self) -> usize {
        self.slots().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_from_inconsistent_length() {
        let grid_creation = Grid::from("ABC\n.#");
        let expected_err = Err(String::from(
            "Inconsistent number of columns: Row #1 has 2 columns but row #0 has 3",
        ));
        assert_eq!(expected_err, grid_creation);
    }

    #[test]
    fn grid_from_invalid_letter() {
        let grid_creation = Grid::from("ABC\n.#@");
        let expected_err = Err(String::from("Invalid value at row #1: @"));
        assert_eq!(expected_err, grid_creation);
    }

    #[test]
    fn grid_row_count() {
        let grid = Grid::from("A\nB").unwrap();
        assert_eq!(2, grid.row_count())
    }

    #[test]
    fn grid_column_count() {
        let grid = Grid::from("A\nB").unwrap();
        assert_eq!(1, grid.column_count())
    }

    #[test]
    fn grid_slots_simple() {
        let grid = Grid::from("...\n...\n...").unwrap();
        let actual_slots = grid.slots();
        let expected_slots = vec![
            Slot::across(0, 0, 3, 0),
            Slot::across(1, 0, 3, 1),
            Slot::across(2, 0, 3, 2),
            Slot::down(3, 0, 3, 0),
            Slot::down(4, 0, 3, 1),
            Slot::down(5, 0, 3, 2),
        ];
        assert_eq!(expected_slots, actual_slots)
    }

    #[test]
    fn grid_slots_asymmetrical() {
        let grid = Grid::from("...\n...").unwrap();
        let actual_slots = grid.slots();
        let expected_slots = vec![
            Slot::across(0, 0, 3, 0),
            Slot::across(1, 0, 3, 1),
            Slot::down(2, 0, 2, 0),
            Slot::down(3, 0, 2, 1),
            Slot::down(4, 0, 2, 2),
        ];
        assert_eq!(expected_slots, actual_slots)
    }

    #[test]
    fn grid_slots_with_blocks() {
        let grid = Grid::from(".#.\n...\n..#").unwrap();
        let actual_slots = grid.slots();
        let expected_slots = vec![
            Slot::across(0, 0, 3, 1),
            Slot::across(1, 0, 2, 2),
            Slot::down(2, 0, 3, 0),
            Slot::down(3, 1, 3, 1),
            Slot::down(4, 0, 2, 2),
        ];
        assert_eq!(expected_slots, actual_slots)
    }

    #[test]
    fn grid_slots_empty() {
        let grid = Grid::from("").unwrap();
        let actual_slots = grid.slots();
        let expected_slots: Vec<Slot> = vec![];
        assert_eq!(expected_slots, actual_slots);
    }
}
