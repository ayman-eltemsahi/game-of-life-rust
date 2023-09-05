mod def;
mod draw;
mod game;

use def::Game;
use draw::draw_game;
use game::run_round;
use std::time::Instant;
use std::{thread, time};

fn main() {
    let now = Instant::now();

    let n: usize = 50;
    let sleep_time = time::Duration::from_millis(150);
    let mut game = Game::new_with_random_live_cells(n, 0.1);
    draw::draw_game(&game);

    for _ in 0..100000 {
        thread::sleep(sleep_time);
        game = run_round(&game);
        draw_game(&game);
    }

    println!("Elapsed: {} ms", now.elapsed().as_millis());
}
