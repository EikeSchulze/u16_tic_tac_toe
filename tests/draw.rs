use u16_tic_tac_toe::{Grid, TOP_LEFT, FieldState, TOP_RIGHT, TOP, LEFT, MIDDLE, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT, GridState};

/// XOX\
/// XOO\
/// OXX
#[test]
fn test_draw_1() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerO);
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerX);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerO);
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);
    grid.set(BOTTOM, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.evaluate(), GridState::Draw);
}

/// XOX\
/// OOX\
/// XXO
#[test]
fn test_draw_2() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerO);
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerO);
    grid.set(MIDDLE, FieldState::PlayerO);
    grid.set(RIGHT, FieldState::PlayerX);
    grid.set(BOTTOM_LEFT, FieldState::PlayerX);
    grid.set(BOTTOM, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerO);

    assert_eq!(grid.evaluate(), GridState::Draw);
}
