#[derive(Clone)]
pub struct Game {
  width: i32,
  height: i32,
}

impl Game {
  pub fn new(width: i32, height: i32, seed: i32) -> Game {
    Game {
      width,
      height,
    }
  }

  pub fn tick(&self) -> Game {
    self.clone()
  }
}
