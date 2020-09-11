/// The state of a field in the [Grid](../struct.Grid.html).
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
