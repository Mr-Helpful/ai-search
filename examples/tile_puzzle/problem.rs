use ai_search::prelude::*;
use rand::Rng;
use std::{
  convert::Infallible,
  fmt::Display,
  ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TilePuzzle {
  pub blank_pos: (usize, usize),
  pub board: [u64; 16],
}

impl Default for TilePuzzle {
  fn default() -> Self {
    Self {
      blank_pos: (0, 0),
      board: std::array::from_fn(|i| i as u64),
    }
  }
}

impl Index<(usize, usize)> for TilePuzzle {
  type Output = u64;
  fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
    &self.board[i + j * 4]
  }
}

impl IndexMut<(usize, usize)> for TilePuzzle {
  fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
    &mut self.board[i + j * 4]
  }
}

impl Display for TilePuzzle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for i in 0..4 {
      for j in 0..4 {
        let item = self[(j, i)];
        if item == 0 {
          write!(f, "   ")?;
        } else {
          write!(f, "{:2} ", item)?;
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

impl TilePuzzle {
  pub fn is_solved(&self) -> bool {
    self.board == TilePuzzle::default().board
  }

  pub fn shuffle(&mut self, rng: &mut (impl Rng + ?Sized), n: usize) {
    for _ in 0..n {
      let actions = self.actions();
      let action = actions[rng.gen_range(0..actions.len())];
      *self = self.result(&action).unwrap();
    }
  }

  pub fn shuffled(rng: &mut (impl Rng + ?Sized), n: usize) -> Self {
    let mut puzzle = Self::default();
    puzzle.shuffle(rng, n);
    puzzle
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Direction::Up => write!(f, "Move empty space Up"),
      Direction::Down => write!(f, "Move empty space Down"),
      Direction::Left => write!(f, "Move empty space Left"),
      Direction::Right => write!(f, "Move empty space Right"),
    }
  }
}

impl Direction {
  fn offset(&self) -> (isize, isize) {
    match self {
      Direction::Up => (0, -1),
      Direction::Down => (0, 1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    }
  }
}

impl State for TilePuzzle {
  type Error = Infallible;
  type Observation = TilePuzzle;

  type ObserveError = Infallible;
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    Ok(*self)
  }

  type Action = Direction;
  type ActionIter = Vec<Direction>;
  fn actions(&self) -> Self::ActionIter {
    let mut actions = Vec::new();
    if self.blank_pos.1 > 0 {
      actions.push(Direction::Up);
    }
    if self.blank_pos.1 < 3 {
      actions.push(Direction::Down);
    }
    if self.blank_pos.0 > 0 {
      actions.push(Direction::Left);
    }
    if self.blank_pos.0 < 3 {
      actions.push(Direction::Right);
    }
    actions
  }

  type ResultError = Infallible;
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    let mut board = *self;
    let (i, j) = board.blank_pos;
    let (ox, oy) = action.offset();
    let (x, y) = (
      i.checked_add_signed(ox).unwrap(),
      j.checked_add_signed(oy).unwrap(),
    );
    let tmp = board[(x, y)];
    board[(x, y)] = board[(i, j)];
    board[(i, j)] = tmp;
    Ok(TilePuzzle {
      blank_pos: (x, y),
      ..board
    })
  }
}
