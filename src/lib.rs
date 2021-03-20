#[derive(Clone)]
pub struct Game {
  width: u16,
  height: u16,
}

impl Game {
  pub fn new(width: u16, height: u16, seed: i32) -> Game {
    Game {
      width,
      height,
    }
  }

  pub fn tick(&self) -> Game {
    self.clone()
  }

  pub fn row_columns(&self) -> Vec<Vec<Cell>> {
    (0..self.height)
        .map(|y| {
          (0..self.width)
              .map(|x| {
                Cell { alive: false }
              })
              .collect()
        })
        .collect()
  }
}


pub struct Cell {
  pub alive: bool,
}
