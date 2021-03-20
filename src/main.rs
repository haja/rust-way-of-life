use std::thread;
use std::time::Duration;

use rust_way_of_life::Game;

fn main() {
  let mut game = Game::new(10, 20, 1234);
  loop {
    print(&game);
    game = game.tick();
    thread::sleep(Duration::from_secs(1));
  }
}

fn print(game: &Game) {
  println!("test");
}
