use crate::pos::Pos;

/// The minimal length of a slot.
pub const MIN_LEN: usize = 2;

/// The definition of a group of contiguous cells.
#[derive(Debug, PartialEq)]
pub struct Slot {
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
    fn new(start: usize, end: usize, offset: usize, is_down: bool) -> Self {
        Slot {
            start,
            end,
            offset,
            is_down,
        }
    }

    /// Creates a new across Slot
    pub fn across(start_column: usize, end_column: usize, row: usize) -> Self {
        Slot::new(start_column, end_column, row, false)
    }

    /// Creates a new down Slot
    pub fn down(start_row: usize, end_row: usize, column: usize) -> Self {
        Slot::new(start_row, end_row, column, true)
    }

    /// Returns the length of this slot.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the positions of the cells of this slot.
    pub fn positions(&self) -> Vec<Pos> {
        (self.start..self.end)
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
        let slot = Slot::across(1, 4, 1);
        let actual_positions = slot.positions();
        let expected_positions = vec![Pos::new(1, 1), Pos::new(2, 1), Pos::new(3, 1)];
        assert_eq!(expected_positions, actual_positions);
    }

    #[test]
    fn slot_positions_down() {
        let slot = Slot::down(1, 4, 1);
        let actual_positions = slot.positions();
        let expected_positions = vec![Pos::new(1, 1), Pos::new(1, 2), Pos::new(1, 3)];
        assert_eq!(expected_positions, actual_positions);
    }
}
