#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! This crate defines a representation of a tic-tac-toe grid, which fits into two bytes.
//! Each field in a tic-tac-toe grid can have three different states (X, O, empty).
//! Therefore there are 3^9 possible states, which can be enumerated with a u16.
//! ```
//! # use u16_tic_tac_toe::Grid;
//! # use core::mem::size_of;
//! assert!(3u16.checked_pow(9).unwrap() < u16::MAX);
//! assert_eq!(size_of::<Grid>(), size_of::<u16>());
//! ```

use core::convert::TryFrom;

const BASE: u16 = 3u16;
const MAX_VALUE: u16 = 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3;

/// The index for the top left field of a grid.
pub const TOP_LEFT: GridIndex = GridIndex(0);
/// The index for the top field of a grid.
pub const TOP: GridIndex = GridIndex(1);
/// The index for the top right field of a grid.
pub const TOP_RIGHT: GridIndex = GridIndex(2);
/// The index for the left field of a grid.
pub const LEFT: GridIndex = GridIndex(3);
/// The index for the middle field of a grid.
pub const MIDDLE: GridIndex = GridIndex(4);
/// The index for the right field of a grid.
pub const RIGHT: GridIndex = GridIndex(5);
/// The index for the bottom left field of a grid.
pub const BOTTOM_LEFT: GridIndex = GridIndex(6);
/// The index for the bottom field of a grid.
pub const BOTTOM: GridIndex = GridIndex(7);
/// The index for the bottom right field of a grid.
pub const BOTTOM_RIGHT: GridIndex = GridIndex(8);

/// The state of a field in the [Grid](struct.Grid.html).
/// This defines whether a field is either blank, occupied by player X or occupied by player O.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum FieldState {
    /// The field is blank and therefore not occupied by any player.
    Blank = 0,
    /// The field is occupied by player X.
    PlayerX = 1,
    /// The field is occupied by player O.
    PlayerO = 2,
}

/// The state of the [Grid](struct.Grid.html).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GridState {
    /// No player has a winning combination and there are still blank fields on the grid.
    Unfinished,
    /// Player X has a winning combination.
    XWins,
    /// Player O has a winning combination.
    OWins,
    /// Both players (X and O) have winning combinations.
    BothWin,
    /// No player has a winning combination, but there are no blank fields left on the grid.
    Draw,
}

/// The field of play for a tic-tac-toe game.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Grid {
    grid: u16
}

impl Into<u16> for &Grid {
    fn into(self) -> u16 {
        self.grid
    }
}

/// A value that is too big to represent a [Grid](struct.Grid.html).
pub type TooBigValue = u16;

impl TryFrom<u16> for Grid {
    type Error = TooBigValue;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < MAX_VALUE {
            Ok(Grid { grid: value })
        } else {
            Err(value)
        }
    }
}

impl Grid {
    /// Creates a new Grid where every field has the state [Blank](enum.FieldState.html).
    /// # Example
    /// ```
    /// # use u16_tic_tac_toe::{Grid, TOP_LEFT, FieldState, TOP, TOP_RIGHT, LEFT, MIDDLE, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT};
    /// let grid = Grid::blank_grid();
    /// assert_eq!(grid.get(TOP_LEFT), FieldState::Blank);
    /// assert_eq!(grid.get(TOP), FieldState::Blank);
    /// assert_eq!(grid.get(TOP_RIGHT), FieldState::Blank);
    /// assert_eq!(grid.get(LEFT), FieldState::Blank);
    /// assert_eq!(grid.get(MIDDLE), FieldState::Blank);
    /// assert_eq!(grid.get(RIGHT), FieldState::Blank);
    /// assert_eq!(grid.get(BOTTOM_LEFT), FieldState::Blank);
    /// assert_eq!(grid.get(BOTTOM), FieldState::Blank);
    /// assert_eq!(grid.get(BOTTOM_RIGHT), FieldState::Blank);
    /// ```
    pub const fn blank_grid() -> Self {
        Grid { grid: 0 }
    }

    /// Returns the [FieldState](enum.FieldState.html) of the grid at the given [GridIndex](struct.GridIndex.html).
    pub fn get(&self, GridIndex(index): GridIndex) -> FieldState {
        let divisor = BASE.pow(index as u32);
        let value = self.grid / divisor;
        let value = value % BASE;
        match value {
            0 => FieldState::Blank,
            1 => FieldState::PlayerX,
            2 => FieldState::PlayerO,
            _ => unreachable!()
        }
    }

    /// Sets the [FieldState](enum.FieldState.html) of the grid at the given [GridIndex](struct.GridIndex.html) to the given field state.
    /// # Examples
    /// ```
    /// # use u16_tic_tac_toe::{Grid, MIDDLE, FieldState};
    /// let mut grid = Grid::blank_grid();
    /// grid.set(MIDDLE, FieldState::PlayerX);
    /// assert_eq!(grid.get(MIDDLE), FieldState::PlayerX);
    /// ```
    pub fn set(&mut self, GridIndex(index): GridIndex, value: FieldState) {
        let new_value = value as u16;
        let divisor = BASE.pow(index as u32);
        let lower_part = self.grid % divisor;
        let upper_part = self.grid / (divisor * BASE);
        self.grid = lower_part + (divisor * new_value) + (upper_part * (divisor * BASE));
    }

    /// Evaluates the [GridState](enum.GridState.html) of the grid.
    /// # Examples
    /// ```
    /// # use u16_tic_tac_toe::{Grid, LEFT, FieldState, MIDDLE, RIGHT, GridState, TOP, BOTTOM};
    /// let blank_grid = Grid::blank_grid();
    /// assert_eq!(blank_grid.evaluate(), GridState::Unfinished);
    ///
    /// let mut grid = Grid::blank_grid();
    /// grid.set(LEFT, FieldState::PlayerX);
    /// grid.set(MIDDLE, FieldState::PlayerX);
    /// grid.set(RIGHT, FieldState::PlayerX);
    /// assert_eq!(grid.evaluate(), GridState::XWins);
    ///
    /// grid.set(TOP, FieldState::PlayerO);
    /// grid.set(MIDDLE, FieldState::PlayerO);
    /// grid.set(BOTTOM, FieldState::PlayerO);
    /// assert_eq!(grid.evaluate(), GridState::OWins);
    /// ```
    pub fn evaluate(&self) -> GridState {
        let top_left = self.get(TOP_LEFT);
        let top = self.get(TOP);
        let top_right = self.get(TOP_RIGHT);
        let left = self.get(LEFT);
        let middle = self.get(MIDDLE);
        let right = self.get(RIGHT);
        let bottom_left = self.get(BOTTOM_LEFT);
        let bottom = self.get(BOTTOM);
        let bottom_right = self.get(BOTTOM_RIGHT);

        use FieldState::{PlayerX, PlayerO, Blank};

        let x_win = (top_left == PlayerX && top == PlayerX && top_right == PlayerX)
            || (left == PlayerX && middle == PlayerX && right == PlayerX)
            || (bottom_left == PlayerX && bottom == PlayerX && bottom_right == PlayerX)
            || (top_left == PlayerX && left == PlayerX && bottom_left == PlayerX)
            || (top == PlayerX && middle == PlayerX && bottom == PlayerX)
            || (top_right == PlayerX && right == PlayerX && bottom_right == PlayerX)
            || (top_left == PlayerX && middle == PlayerX && bottom_right == PlayerX)
            || (top_right == PlayerX && middle == PlayerX && bottom_left == PlayerX);
        let o_win = (top_left == PlayerO && top == PlayerO && top_right == PlayerO)
            || (left == PlayerO && middle == PlayerO && right == PlayerO)
            || (bottom_left == PlayerO && bottom == PlayerO && bottom_right == PlayerO)
            || (top_left == PlayerO && left == PlayerO && bottom_left == PlayerO)
            || (top == PlayerO && middle == PlayerO && bottom == PlayerO)
            || (top_right == PlayerO && right == PlayerO && bottom_right == PlayerO)
            || (top_left == PlayerO && middle == PlayerO && bottom_right == PlayerO)
            || (top_right == PlayerO && middle == PlayerO && bottom_left == PlayerO);

        match (x_win, o_win) {
            (true, true) => GridState::BothWin,
            (true, false) => GridState::XWins,
            (false, true) => GridState::OWins,
            (false, false) => {
                if top_left == Blank || top == Blank || top_right == Blank || left == Blank || middle == Blank || right == Blank || bottom_left == Blank || bottom == Blank || bottom_right == Blank {
                    GridState::Unfinished
                } else {
                    GridState::Draw
                }
            }
        }
    }
}

/// This struct represents a valid index for a [Grid](struct.Grid.html).
/// Valid grid indexes are in the range of [0, 8].
/// # Examples
/// ```
/// # use u16_tic_tac_toe::GridIndex;
/// # use core::convert::TryFrom;
/// assert!(GridIndex::try_from(0u8).is_ok());
/// assert!(GridIndex::try_from(1u8).is_ok());
/// assert!(GridIndex::try_from(2u8).is_ok());
/// assert!(GridIndex::try_from(3u8).is_ok());
/// assert!(GridIndex::try_from(4u8).is_ok());
/// assert!(GridIndex::try_from(5u8).is_ok());
/// assert!(GridIndex::try_from(6u8).is_ok());
/// assert!(GridIndex::try_from(7u8).is_ok());
/// assert!(GridIndex::try_from(8u8).is_ok());
/// assert!(GridIndex::try_from(9u8).is_err());
/// ```
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GridIndex(u8);

/// A value that is too big to be a [GridIndex](struct.GridIndex.html).
pub type TooBigIndex = u8;

impl GridIndex {
    /// Tries to create a GridIndex with the given value.
    /// A value in the range of [0, 8] will return a valid GridIndex.
    /// A value outside of that range will return a TooBigIndex error.
    pub const fn try_new(value: u8) -> Result<Self, TooBigIndex> {
        if value < 9 {
            Ok(GridIndex(value))
        } else {
            Err(value)
        }
    }
}

impl Into<u8> for GridIndex {
    fn into(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for GridIndex {
    type Error = TooBigIndex;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}
