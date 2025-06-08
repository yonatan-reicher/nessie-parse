pub type Row = u16;
pub type Col = u16;

/// A position in some source code string.
/// Positions save the offset, the row and the column. That means that a
/// position is only valid for a specific source code string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    /// Starts at 0.
    pub offset: usize,
    /// Starts at 1.
    pub row: Row,
    /// Starts at 1.
    pub col: Col,
}

impl Pos {
    /// Creates a new position at the start of source code.
    pub const fn start() -> Self {
        Pos {
            offset: 0,
            row: 1,
            col: 1,
        }
    }

    /// Returns this position, one character to the right.
    pub const fn right(self) -> Self {
        Pos {
            offset: self.offset + 1,
            row: self.row,
            col: self.col + 1,
        }
    }

    /// Returns this position, one line down, *and with the column reset to 1*.
    pub const fn down(self) -> Self {
        Pos {
            offset: self.offset + 1,
            row: self.row + 1,
            col: 1,
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Pos::start()
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Pos { offset, row, col } = *self;
        write!(f, "Pos(offset {offset} column {col} row {row})")
    }
}
