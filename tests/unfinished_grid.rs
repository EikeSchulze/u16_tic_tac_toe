use u16_tic_tac_toe::{Grid, TOP_LEFT, FieldState, TOP_RIGHT, TOP, LEFT, MIDDLE, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT, GridState};

#[test]
fn test_empty_grid() {
    let grid = Grid::blank_grid();
    assert_eq!(grid.get(TOP_LEFT), FieldState::Blank);
    assert_eq!(grid.get(TOP), FieldState::Blank);
    assert_eq!(grid.get(TOP_RIGHT), FieldState::Blank);
    assert_eq!(grid.get(LEFT), FieldState::Blank);
    assert_eq!(grid.get(MIDDLE), FieldState::Blank);
    assert_eq!(grid.get(RIGHT), FieldState::Blank);
    assert_eq!(grid.get(BOTTOM_LEFT), FieldState::Blank);
    assert_eq!(grid.get(BOTTOM), FieldState::Blank);
    assert_eq!(grid.get(BOTTOM_RIGHT), FieldState::Blank);
    assert_eq!(grid.evaluate(), GridState::Unfinished);
}

/// XOX\
/// X O\
/// OXX
#[test]
fn test_close_to_full_grid() {
    let mut grid = Grid::blank_grid();
    grid.set(TOP_LEFT, FieldState::PlayerX);
    grid.set(TOP, FieldState::PlayerO);
    grid.set(TOP_RIGHT, FieldState::PlayerX);
    grid.set(LEFT, FieldState::PlayerX);
    grid.set(RIGHT, FieldState::PlayerO);
    grid.set(BOTTOM_LEFT, FieldState::PlayerO);
    grid.set(BOTTOM, FieldState::PlayerX);
    grid.set(BOTTOM_RIGHT, FieldState::PlayerX);

    assert_eq!(grid.get(TOP_LEFT), FieldState::PlayerX);
    assert_eq!(grid.get(TOP), FieldState::PlayerO);
    assert_eq!(grid.get(TOP_RIGHT), FieldState::PlayerX);
    assert_eq!(grid.get(LEFT), FieldState::PlayerX);
    assert_eq!(grid.get(MIDDLE), FieldState::Blank);
    assert_eq!(grid.get(RIGHT), FieldState::PlayerO);
    assert_eq!(grid.get(BOTTOM_LEFT), FieldState::PlayerO);
    assert_eq!(grid.get(BOTTOM), FieldState::PlayerX);
    assert_eq!(grid.get(BOTTOM_RIGHT), FieldState::PlayerX);
    assert_eq!(grid.evaluate(), GridState::Unfinished);
}
