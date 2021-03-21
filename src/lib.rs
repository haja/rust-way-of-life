use rand_xoshiro::Xoshiro256PlusPlus;
use rand_xoshiro::rand_core::{SeedableRng, RngCore};

#[derive(Clone)]
pub struct Game {
  cells: Box<Vec<Vec<Cell>>>
}

impl Game {
  pub fn new(width: u16, height: u16, seed: u64) -> Game {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    let cells = Box::new(
      (0..height)
          .map(|y| {
            (0..width)
                .map(|x| {
                  let alive = rng.next_u32() % 2 == 0;
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

#[derive(Clone)]
pub struct Cell {
  pub alive: bool,
}
