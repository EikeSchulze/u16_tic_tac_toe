use core::convert::TryFrom;

use crate::{BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, FieldState, GridIndex, GridState, LEFT, MIDDLE, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};
use crate::field_state::FieldState::{Blank, PlayerO, PlayerX};
use crate::grid_state::GridState::{BothWin, Draw, OWins, Unfinished, XWins};

const BASE: u16 = 3u16;
const MAX_VALUE: u16 = 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3 * 3;

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
    /// Creates a new Grid where every field has the state [Blank](../enum.FieldState.html).
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
    pub fn blank_grid() -> Self {
        Grid { grid: 0 }
    }

    /// Returns the [FieldState](../enum.FieldState.html) of the grid at the given [GridIndex](../struct.GridIndex.html).
    pub fn get(&self, index: GridIndex) -> FieldState {
        let index: u8 = index.into();
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

    /// Sets the [FieldState](../enum.FieldState.html) of the grid at the given [GridIndex](../struct.GridIndex.html) to the given field state.
    /// # Examples
    /// ```
    /// # use u16_tic_tac_toe::{Grid, MIDDLE, FieldState};
    /// let mut grid = Grid::blank_grid();
    /// grid.set(MIDDLE, FieldState::PlayerX);
    /// assert_eq!(grid.get(MIDDLE), FieldState::PlayerX);
    /// ```
    pub fn set(&mut self, index: GridIndex, value: FieldState) {
        let index: u8 = index.into();
        let new_value = value as u16;
        let divisor = BASE.pow(index as u32);
        let lower_part = self.grid % divisor;
        let upper_part = self.grid / (divisor * BASE);
        self.grid = lower_part + (divisor * new_value) + (upper_part * (divisor * BASE));
    }

    /// Evaluates the [GridState](../enum.GridState.html) of the grid.
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
            (true, true) => BothWin,
            (true, false) => XWins,
            (false, true) => OWins,
            (false, false) => {
                if top_left == Blank || top == Blank || top_right == Blank || left == Blank || middle == Blank || right == Blank || bottom_left == Blank || bottom == Blank || bottom_right == Blank {
                    Unfinished
                } else {
                    Draw
                }
            }
        }
    }
}
