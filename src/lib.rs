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

pub use field_state::FieldState;
pub use grid::{Grid, TooBigValue};
pub use grid_state::GridState;
pub use index::{BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, GridIndex, LEFT, MIDDLE, RIGHT, TooBigIndex, TOP, TOP_LEFT, TOP_RIGHT};

mod field_state;
mod grid;
mod grid_state;
mod index;
