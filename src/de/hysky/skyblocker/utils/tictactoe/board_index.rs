//! Ported from de.hysky.skyblocker.utils.tictactoe.BoardIndex by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record BoardIndex(int row, int column)
#[derive(Clone, Debug, PartialEq)]
pub struct BoardIndex {
    /// java: int row
    pub row: i32,
    /// java: int column
    pub column: i32,
}

impl BoardIndex {
    /// java: the canonical record constructor
    pub fn new(mut row: i32, mut column: i32) -> Self {
        Self { row, column }
    }

    /// java: the record accessor `row()`
    pub fn row(&self) -> i32 {
        self.row
    }

    /// java: the record accessor `column()`
    pub fn column(&self) -> i32 {
        self.column
    }

}
