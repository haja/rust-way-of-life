use std::thread;
use std::time::Duration;

use rust_way_of_life::Game;

fn main() {
  let mut game = Game::new(10, 5, 1234);
  loop {
    print(&game);
    game = game.tick();
    thread::sleep(Duration::from_secs(1));
  }
}

fn print(game: &Game) {
  println!("we are live!");
  game.row_columns().iter().for_each(|row| {
    let str: String = row.iter().map(|cell| {
      if cell.alive {
        'x'
      } else {
        '.'
      }
    }).collect();
    println!("{}", str);
  });
}
