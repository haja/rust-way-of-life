use std::fmt;

use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
  cells: Vec<Vec<Cell>>,
  iteration_count: u64,
  wrapping: bool,
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let state: String = self.row_columns().iter().map(|row| {
      let mut mapped = row.iter().map(|cell| {
        if cell.alive {
          '#'
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
  pub fn new(width: u16, height: u16, seed: u64, wrapping: bool) -> Game {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
    let cells =
        (0..height)
            .map(|_y| {
              (0..width)
                  .map(|_x| {
                    let alive = rng.next_u32() % 2 == 0;
                    Cell { alive }
                  })
                  .collect()
            })
            .collect();
    Game {
      iteration_count: 0,
      cells,
      wrapping,
    }
  }

  pub fn from_specific(layout: &str, wrapping: bool) -> Game {
    let lines: Vec<&str> = layout.lines().collect();
    let width = lines[0].len();

    let cells = lines.iter()
        .map(|line| {
          if line.len() != width {
            panic!("layout invalid, all lines need to be of same length");
          } else {
            line.chars().map(to_cell).collect()
          }
        })
        .collect();
    Game {
      iteration_count: 0,
      cells,
      wrapping,
    }
  }

  pub fn tick(&self) -> Game {
    let mut next = self.clone();
    next.iteration_count = self.iteration_count + 1;
    next.cells =
        self.cells.iter()
            .enumerate()
            .map(|(y, row)| {
              row.iter()
                  .enumerate()
                  .map(|(x, old_cell)| {
                    let neighbours = get_neighbours(self, x, y);
                    Cell {
                      alive: staying_alive(old_cell, &neighbours),
                    }
                  })
                  .collect()
            })
            .collect();
    next
  }

  pub fn row_columns(&self) -> &Vec<Vec<Cell>> {
    &self.cells
  }
}

fn staying_alive(cell: &Cell, neighbours: &[&Cell]) -> bool {
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
  let same_row = get_two(game, x, y);

  vec![prev_row, same_row, next_row].concat()
}

fn get_two(game: &Game, x: usize, y: usize) -> Vec<&Cell> {
  [x as i32 - 1, x as i32 + 1]
      .iter()
      .map(|xi| {
        get_if_positive(&game.cells[y], *xi)
      })
      .flatten()
      .collect()
}

fn get_three(game: &Game, x: usize, y: i32) -> Vec<&Cell> {
  if game.wrapping {
    get_wrapping(&game.cells, x, y)
  } else {
    get_or_empty(&game.cells, x, y)
  }
}

fn get_wrapping(cells: &[Vec<Cell>], x: usize, y: i32) -> Vec<&Cell> {
  let y_wrapped = wrap(y, cells.len());
  cells
      .get(y_wrapped)
      .map(|row| {
        get_three_of_row(x, row)
      })
      .unwrap_or_default()
}

fn wrap(idx: i32, len: usize) -> usize {
  let len = len as i32;
  if idx < 0 {
    (len + idx % len) as usize
  } else {
    (idx % len) as usize
  }
}

fn get_or_empty(vec: &[Vec<Cell>], x: usize, y: i32) -> Vec<&Cell> {
  if y >= 0 {
    vec
        .get(y as usize)
        .map(|row| {
          get_three_of_row(x, row)
        })
        .unwrap_or_default()
  } else {
    Vec::new()
  }
}

fn get_three_of_row(x: usize, row: &[Cell]) -> Vec<&Cell> {
  ((x as i32 - 1)..=(x as i32 + 1))
      .map(|xi| {
        get_if_positive(row, xi)
      })
      .flatten()
      .collect()
}

fn get_if_positive<T>(vec: &[T], idx: i32) -> Option<&T> {
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
    let game = Game::new(2, 1, 1, false);

    assert_eq!(game.row_columns().len(), 1);
  }

  #[test]
  fn new_game_should_contain_one_column() {
    let game = Game::new(1, 2, 1, false);

    assert_eq!(game.row_columns()[0].len(), 1);
  }

  #[test]
  fn new_game_all_columns_should_be_same_length() {
    let game = Game::new(1, 2, 1, false);

    assert_eq!(game.row_columns()[0].len(), game.row_columns()[1].len());
  }

  #[test]
  fn game_from_specific_alive() {
    let game = Game::from_specific("#", false);

    assert_eq!(game.row_columns()[0][0].alive, true);
  }

  #[test]
  fn game_from_specific_dead() {
    let game = Game::from_specific(".", false);

    assert_eq!(game.row_columns()[0][0].alive, false);
  }

  #[test]
  fn game_from_specific_two_columns() {
    let game = Game::from_specific(".#", false);

    assert_eq!(game.row_columns()[0][0].alive, false);
    assert_eq!(game.row_columns()[0][1].alive, true);
  }

  #[test]
  fn game_from_specific_two_rows() {
    let game = Game::from_specific("#\n.", false);

    assert_eq!(game.row_columns()[0][0].alive, true);
    assert_eq!(game.row_columns()[1][0].alive, false);
  }

  #[test]
  #[should_panic]
  fn game_from_specific_invalid_line_lengths_should_panic() {
    Game::from_specific(".\n..", false);
  }

  #[test]
  fn dead_alone_should_stay_dead() {
    let initial = Game::from_specific(".", false);

    let result = initial.tick();

    assert_eq!(result.row_columns()[0][0].alive, false);
  }

  #[test]
  fn alive_alone_should_die() {
    let initial = Game::from_specific("#", false);

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

    assert_cells(result, beehive());
  }

  #[test]
  fn blinker_should_stay_alive_2_ticks() {
    let initial = blinker();

    let result = initial.tick().tick();

    assert_cells(result, blinker());
  }

  #[test]
  fn blinker_should_blink() {
    let initial = blinker();

    let result = initial.tick();

    assert_cells(result, blinker_vert());
  }

  #[test]
  fn wrapping_blinker_top_should_blink() {
    let initial = blinker_wrapped();

    let result = initial.tick();

    assert_cells(result, blinker_vert_wrapped());
  }

  #[test]
  fn wrapping_blinker_top_stay_alive_2_ticks() {
    let initial = blinker_wrapped();

    let result = initial.tick().tick();

    assert_cells(result, blinker_wrapped());
  }

  #[test]
  fn wrapping_blinker_left_should_blink() {
    let initial = blinker_wrapped_left();

    let result = initial.tick();

    assert_cells(result, blinker_vert_wrapped_left());
  }

  #[test]
  fn wrapping_blinker_left_stay_alive_2_ticks() {
    let initial = blinker_wrapped_left();

    let result = initial.tick().tick();

    assert_cells(result, blinker_wrapped_left());
  }

  #[test]
  fn iteration_should_count_up() {
    let initial = Game::new(1, 1, 1, false);

    let result = initial.tick();

    assert_eq!(result.iteration_count, 1);
  }

  fn block() -> Game {
    Game::from_specific(
      "\
....
.##.
.##.
....",
      false,
    )
  }

  fn beehive() -> Game {
    Game::from_specific(
      "\
......
..##..
.#..#.
..##..
......",
      false,
    )
  }

  fn blinker() -> Game {
    Game::from_specific(
      "\
.....
..#..
..#..
..#..
.....",
      false,
    )
  }

  fn blinker_vert() -> Game {
    Game::from_specific(
      "\
.....
.....
.###.
.....
.....",
      false,
    )
  }

  fn blinker_wrapped() -> Game {
    Game::from_specific(
      "\
..#..
..#..
.....
.....
..#..",
      true,
    )
  }

  fn blinker_vert_wrapped() -> Game {
    Game::from_specific(
      "\
.###.
.....
.....
.....
.....",
      true,
    )
  }


  fn blinker_wrapped_left() -> Game {
    Game::from_specific(
      "\
.....
#....
#....
#....
.....",
      false,
    )
  }

  fn blinker_vert_wrapped_left() -> Game {
    Game::from_specific(
      "\
.....
.....
##..#
.....
.....",
      false,
    )
  }

  fn assert_cells(result: Game, expected: Game) {
    println!("result:{}\nexpected:{}", result, expected);
    assert_eq!(result.cells, expected.cells);
  }
}
