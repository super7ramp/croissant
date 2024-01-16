use crate::pos::Pos;

/// The minimal length of a slot.
pub const MIN_LEN: usize = 2;

/// The definition of a group of contiguous cells.
#[derive(Debug, PartialEq)]
pub struct Slot {
    /// The index of this slot in the grid's list of slots.
    index: usize,
    /// The start of the varying coordinate.
    start: usize,
    /// The end of the varying coordinate.
    end: usize,
    /// The fixed coordinate.
    offset: usize,
    /// Whether this is a down slot.
    is_down: bool,
}

impl Slot {
    /// Creates a new Slot
    fn new(index: usize, start: usize, end: usize, offset: usize, is_down: bool) -> Self {
        Slot {
            index,
            start,
            end,
            offset,
            is_down,
        }
    }

    /// Creates a new across Slot
    pub fn across(index: usize, start_column: usize, end_column: usize, row: usize) -> Self {
        Slot::new(index, start_column, end_column, row, false)
    }

    /// Creates a new down Slot
    pub fn down(index: usize, start_row: usize, end_row: usize, column: usize) -> Self {
        Slot::new(index, start_row, end_row, column, true)
    }

    /// Returns the length of this slot.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the index of this slot in the grid list of slots.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the positions of the cells of this slot.
    pub fn positions(&self) -> Vec<Pos> {
        (0..self.len())
            .map(|i| {
                if self.is_down {
                    Pos::new(self.offset, i)
                } else {
                    Pos::new(i, self.offset)
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slot_positions_across() {
        let slot = Slot::across(42, 0, 3, 1);
        let actual_positions = slot.positions();
        let expected_positions = vec![Pos::new(0, 1), Pos::new(1, 1), Pos::new(2, 1)];
        assert_eq!(expected_positions, actual_positions);
    }

    #[test]
    fn slot_positions_down() {
        let slot = Slot::down(42, 0, 3, 1);
        let actual_positions = slot.positions();
        let expected_positions = vec![Pos::new(1, 0), Pos::new(1, 1), Pos::new(1, 2)];
        assert_eq!(expected_positions, actual_positions);
    }
}