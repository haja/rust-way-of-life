use std::fmt;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
  cells: Box<Vec<Vec<Cell>>>,
  iteration_count: u64,
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let state: String = self.row_columns().iter().map(|row| {
      let mut mapped = row.iter().map(|cell| {
        if cell.alive {
          'x'
        } else {
          '.'
        }
      })
          .collect::<String>();
      mapped.push('\n');
      mapped
    }).collect();
    write!(f, "iteration {}:\n{}", self.iteration_count, state)
  }
}

#[derive(Clone, Debug, PartialEq)]
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
      iteration_count: 0,
      cells,
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
    Game {
      iteration_count: 0,
      cells,
    }
  }

  pub fn tick(&self) -> Game {
    let mut next = self.clone();
    next.iteration_count = self.iteration_count + 1;
    next.cells = Box::new(
      self.cells.iter()
          .enumerate()
          .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, old_cell)| {
                  let neighbours = get_neighbours(self, x, y);
                  let cell = Cell {
                    alive: staying_alive(old_cell, &neighbours),
                  };
                  cell
                })
                .collect()
          })
          .collect()
    );
    next
  }

  pub fn row_columns(&self) -> &Vec<Vec<Cell>> {
    &self.cells
  }
}

fn staying_alive(cell: &Cell, neighbours: &Vec<&Cell>) -> bool {
  let alive_neighbours_count = neighbours
      .iter()
      .filter(|c| c.alive)
      .count();
  if cell.alive {
    alive_neighbours_count == 2 || alive_neighbours_count == 3
  } else {
    alive_neighbours_count == 3
  }
}

fn get_neighbours(game: &Game, x: usize, y: usize) -> Vec<&Cell> {
  let prev_row = get_three(game, x, y as i32 - 1);
  let next_row = get_three(game, x, (y + 1) as i32);
  let mut other_rows = vec![prev_row, next_row].concat();

  let same_row = &game.cells[y];
  [x as i32 - 1, x as i32 + 1]
      .iter()
      .for_each(|xi| {
        if let Some(cell) = get_if_positive(same_row, *xi) {
          other_rows.push(cell);
        }
      });

  other_rows
}

fn get_three(game: &Game, x: usize, y: i32) -> Vec<&Cell> {
  if y >= 0 {
    game.cells
        .get(y as usize)
        .map(|r| get_three_of_row(x, r))
        .unwrap_or(Vec::new())
  } else {
    Vec::new()
  }
}

fn get_three_of_row(x: usize, row: &Vec<Cell>) -> Vec<&Cell> {
  ((x as i32 - 1)..=(x as i32 + 1))
      .map(|xi| {
        get_if_positive(row, xi)
      })
      .flatten()
      .collect()
}

fn get_if_positive<T>(vec: &Vec<T>, idx: i32) -> Option<&T> {
  if idx >= 0 {
    vec.get(idx as usize)
  } else {
    None
  }
}

// TODO implement From instead?
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

  #[test]
  fn dead_alone_should_stay_dead() {
    let initial = Game::from_specific(".");

    let result = initial.tick();

    assert_eq!(result.row_columns()[0][0].alive, false);
  }

  #[test]
  fn alive_alone_should_die() {
    let initial = Game::from_specific("#");

    let result = initial.tick();

    assert_eq!(result.row_columns()[0][0].alive, false);
  }

  #[test]
  fn block_should_stay_alive() {
    let initial = block();

    let result = initial.tick();

    assert_cells(result, block());
  }

  #[test]
  fn beehive_should_stay_alive() {
    let initial = beehive();

    let result = initial.tick();

    println!("inital {}\nresult {}", initial, result);
    assert_cells(result, beehive());
  }

  #[test]
  fn iteration_should_count_up() {
    let initial = Game::new(1, 1, 1);

    let result = initial.tick();

    assert_eq!(result.iteration_count, 1);
  }

  fn block() -> Game {
    Game::from_specific(
      "##
##"
    )
  }

  fn beehive() -> Game {
    Game::from_specific(
      "......
..##..
.#..#.
..##..
......"
    )
  }

  fn assert_cells(result: Game, expected: Game) {
    assert_eq!(result.cells, expected.cells);
  }
}
