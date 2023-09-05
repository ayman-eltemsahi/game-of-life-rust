use rand::Rng;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum CellState {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Debug, Copy)]
pub struct Cell {
    pub state: CellState,
}

#[derive(Clone, Debug)]
pub struct Game {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Game {
    pub fn new(n: usize) -> Game {
        let cells: Vec<Vec<Cell>> = vec![vec![Cell { state: CellState::Dead }; n]; n];
        Game { size: n, cells }
    }

    pub fn new_with_random_live_cells(n: usize, percentage: f32) -> Game {
        let mut game = Game::new(n);
        let mut rng = rand::thread_rng();

        for i in 0..n {
            for j in 0..n {
                if rng.gen_range(0.0..1.0) <= percentage {
                    game.update_cell(i, j, CellState::Alive);
                }
            }
        }

        game
    }

    pub fn update_cell(&mut self, i: usize, j: usize, state: CellState) {
        self.cells[i][j].state = state;
    }

    pub fn is_alive_at(&self, i: usize, j: usize) -> bool {
        self.cells[i][j].state == CellState::Alive
    }
}
