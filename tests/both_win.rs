use u16_tic_tac_toe::{BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, FieldState, Grid, GridState, LEFT, MIDDLE, RIGHT, TOP, TOP_LEFT, TOP_RIGHT};

#[test]
fn test_x_top_row_o_bottom_row() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerX);
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);
    grid.set(BOTTOM, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::BothWin);
}

#[test]
fn test_x_top_row_o_middle_row() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerX);
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::BothWin);
}

#[test]
fn test_x_left_column_o_right_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerX);
    grid.set(BOTTOM_LEFT, FieldState::PlayerX);
    grid.set(TOP_RIGHT, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::BothWin);
}

#[test]
fn test_x_middle_column_o_right_column() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerX);
    grid.set(BOTTOM, FieldState::PlayerX);
    grid.set(TOP_RIGHT, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::BothWin);
}
