/// The state of the [Grid](../struct.Grid.html).
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
