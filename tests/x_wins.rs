use u16_tic_tac_toe::{BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, FieldState, Grid, GridState, LEFT, MIDDLE, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};

#[test]
fn test_x_wins_top_row() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerX);
    grid.set(TOP_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_middle_row() {
    let mut grid = Grid::blank_grid();
    grid.set(LEFT, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerX);
    grid.set(RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_bottom_row() {
    let mut grid = Grid::blank_grid();
    grid.set(BOTTOM_LEFT, FieldState::PlayerX);
    grid.set(BOTTOM, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_left_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerX);
    grid.set(BOTTOM_LEFT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_middle_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerX);
    grid.set(BOTTOM, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_right_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(RIGHT, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_falling_diagonal() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}

#[test]
fn test_x_wins_rising_diagonal() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerX);
    grid.set(BOTTOM_LEFT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::XWins);
}
