#[derive(Clone)]
pub struct Game {
  cells: Box<Vec<Vec<Cell>>>
}

impl Game {
  pub fn new(width: u16, height: u16, seed: u16) -> Game {
    let cells = Box::new(
      (0..height)
          .map(|y| {
            (0..width)
                .map(|x| {
                  let alive = rand::random();
                  Cell { alive }
                })
                .collect()
          })
          .collect()
    );
    Game {
      cells
    }
  }

  pub fn tick(&self) -> Game {
    self.clone()
  }

  pub fn row_columns(&self) -> &Vec<Vec<Cell>> {
    &self.cells
  }
}


fn prand(seed: u16, y: u16, x: u16) -> u16 {
  let (uneven, _) = seed.overflowing_shr(1);
  uneven.wrapping_mul(y + 1).wrapping_mul(x + 1)
}

#[derive(Clone)]
pub struct Cell {
  pub alive: bool,
}
