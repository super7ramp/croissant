/// A cell position in the grid.
#[derive(Debug, PartialEq)]
pub struct Pos {
    column: usize,
    row: usize,
}

impl Pos {
    pub fn new(column: usize, row: usize) -> Self {
        Pos { column, row }
    }
}
