use crate::def::{CellState, Game};

const TRANSITIONS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn count_states(game: &Game, i: usize, j: usize, n: usize, state: CellState) -> usize {
    TRANSITIONS
        .iter()
        .map(|k| ((i as i32 + k.0), (j as i32 + k.1)))
        .filter(|k| k.0 >= 0 && k.1 >= 0 && k.0 < n as i32 && k.1 < n as i32)
        .filter(|k| game.cells[k.0 as usize][k.1 as usize].state == state)
        .count()
}

/**
 * Rules of the game:
 * Any live cell with fewer than two live neighbours dies, as if by underpopulation.
 * Any live cell with two or three live neighbours lives on to the next generation.
 * Any live cell with more than three live neighbours dies, as if by overpopulation.
 * Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 */
pub fn run_round(game: &Game) -> Game {
    let mut updated = Game::new(game.size);

    for i in 0..game.size {
        for j in 0..game.size {
            let alive_count = count_states(&game, i, j, game.size, CellState::Alive);

            match game.cells[i][j].state {
                CellState::Alive => match alive_count {
                    2 | 3 => updated.update_cell(i, j, CellState::Alive),
                    _ => {}
                },
                CellState::Dead => match alive_count {
                    3 => updated.update_cell(i, j, CellState::Alive),
                    _ => {}
                },
            };
        }
    }

    updated
}
