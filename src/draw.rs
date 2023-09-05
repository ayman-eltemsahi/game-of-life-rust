use crate::def::Game;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn draw_game(game: &Game) {
    let n = game.size;

    let mut line = String::from("");
    for i in 0..n {
        for j in 0..n {
            line += if game.is_alive_at(i, j) { "█" } else { " " };
        }

        line += "\n";
    }

    clear_screen();
    println!("{}", line);
}
