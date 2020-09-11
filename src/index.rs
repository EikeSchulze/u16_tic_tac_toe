use core::convert::TryFrom;

/// The index for the top left field of a grid.
pub const TOP_LEFT: GridIndex = GridIndex { value: 0 };
/// The index for the top field of a grid.
pub const TOP: GridIndex = GridIndex { value: 1 };
/// The index for the top right field of a grid.
pub const TOP_RIGHT: GridIndex = GridIndex { value: 2 };
/// The index for the left field of a grid.
pub const LEFT: GridIndex = GridIndex { value: 3 };
/// The index for the middle field of a grid.
pub const MIDDLE: GridIndex = GridIndex { value: 4 };
/// The index for the right field of a grid.
pub const RIGHT: GridIndex = GridIndex { value: 5 };
/// The index for the bottom left field of a grid.
pub const BOTTOM_LEFT: GridIndex = GridIndex { value: 6 };
/// The index for the bottom field of a grid.
pub const BOTTOM: GridIndex = GridIndex { value: 7 };
/// The index for the bottom right field of a grid.
pub const BOTTOM_RIGHT: GridIndex = GridIndex { value: 8 };

/// This struct represents a valid index for a [Grid](../struct.Grid.html).
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
pub struct GridIndex {
    value: u8
}

/// A value that is too big to be a [GridIndex](struct.GridIndex.html).
pub type TooBigIndex = u8;

impl Into<u8> for GridIndex {
    fn into(self) -> u8 {
        self.value
    }
}

impl TryFrom<u8> for GridIndex {
    type Error = TooBigIndex;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 9 {
            Ok(GridIndex { value })
        } else {
            Err(value)
        }
    }
}
