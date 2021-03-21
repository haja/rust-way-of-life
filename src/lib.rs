use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Clone, Debug)]
pub struct Game {
  cells: Box<Vec<Vec<Cell>>>
}

#[derive(Clone, Debug)]
pub struct Cell {
  pub alive: bool,
}

impl Game {
  pub fn new(width: u16, height: u16, seed: u64) -> Game {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    let cells = Box::new(
      (0..height)
          .map(|_y| {
            (0..width)
                .map(|_x| {
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

  pub fn from_specific(layout: &str) -> Game {
    let lines: Vec<&str> = layout.lines().collect();
    let width = lines[0].len();

    let cells = Box::new(lines.iter()
        .map(|line| {
          if line.len() != width {
            panic!("layout invalid, all lines need to be of same length");
          } else {
            line.chars().map(to_cell).collect()
          }
        })
        .collect()
    );
    Game { cells }
  }

  pub fn tick(&self) -> Game {
    self.clone()
  }

  pub fn row_columns(&self) -> &Vec<Vec<Cell>> {
    &self.cells
  }
}

fn to_cell(c: char) -> Cell {
  Cell { alive: c == '#' }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_game_should_contain_one_row() {
    let game = Game::new(2, 1, 1);

    assert_eq!(game.row_columns().len(), 1);
  }

  #[test]
  fn new_game_should_contain_one_column() {
    let game = Game::new(1, 2, 1);

    assert_eq!(game.row_columns()[0].len(), 1);
  }

  #[test]
  fn new_game_all_columns_should_be_same_length() {
    let game = Game::new(1, 2, 1);

    assert_eq!(game.row_columns()[0].len(), game.row_columns()[1].len());
  }

  #[test]
  fn game_from_specific_alive() {
    let game = Game::from_specific("#");

    assert_eq!(game.row_columns()[0][0].alive, true);
  }

  #[test]
  fn game_from_specific_dead() {
    let game = Game::from_specific(".");

    assert_eq!(game.row_columns()[0][0].alive, false);
  }

  #[test]
  fn game_from_specific_two_columns() {
    let game = Game::from_specific(".#");

    assert_eq!(game.row_columns()[0][0].alive, false);
    assert_eq!(game.row_columns()[0][1].alive, true);
  }

  #[test]
  fn game_from_specific_two_rows() {
    let game = Game::from_specific("#\n.");

    assert_eq!(game.row_columns()[0][0].alive, true);
    assert_eq!(game.row_columns()[1][0].alive, false);
  }

  #[test]
  #[should_panic]
  fn game_from_specific_invalid_line_lengths_should_panic() {
    Game::from_specific(".\n..");
  }
}
