use u16_tic_tac_toe::{BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, FieldState, Grid, GridState, LEFT, MIDDLE, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};

#[test]
fn test_x_wins_top_row() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerO);
    grid.set(TOP, FieldState::PlayerO);
    grid.set(TOP_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_middle_row() {
    let mut grid = Grid::blank_grid();
    grid.set(LEFT, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_bottom_row() {
    let mut grid = Grid::blank_grid();
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);
    grid.set(BOTTOM, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_left_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerO);
    grid.set(LEFT, FieldState::PlayerO);
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_middle_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(BOTTOM, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_right_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_RIGHT, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_falling_diagonal() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}

#[test]
fn test_x_wins_rising_diagonal() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_RIGHT, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::OWins);
}
