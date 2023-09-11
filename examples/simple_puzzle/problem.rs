use ai_search::prelude::*;
use std::{convert::Infallible, fmt::Display, vec};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct SimplePuzzle {
  pub n: usize,
}

impl Display for SimplePuzzle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "SimplePuzzle (n = {})", self.n)
  }
}

impl SimplePuzzle {
  pub fn new(n: usize) -> Self {
    Self { n }
  }
  pub fn is_solved(&self) -> bool {
    self.n == 0
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
  Increment,
  Decrement,
}

impl Display for Direction {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Direction::Increment => write!(f, "Increment"),
      Direction::Decrement => write!(f, "Decrement"),
    }
  }
}

impl State for SimplePuzzle {
  type Error = Infallible;
  type Observation = SimplePuzzle;

  type ObserveError = Infallible;
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    Ok(*self)
  }

  type Action = Direction;
  type ActionIter = Vec<Direction>;
  fn actions(&self) -> Self::ActionIter {
    if self.n == 0 {
      vec![Direction::Increment]
    } else {
      vec![Direction::Increment, Direction::Decrement]
    }
  }

  type ResultError = Infallible;
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    match action {
      Direction::Increment => Ok(Self { n: self.n + 1 }),
      Direction::Decrement => Ok(Self { n: self.n - 1 }),
    }
  }
}
