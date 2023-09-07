use super::{Search, State};
use std::collections::VecDeque;

/// A Breadth first traversal of possible states.
///
/// Will only explore states at a given depth after all states at the previous
/// depth have been explored.
pub struct Bfs<S: State> {
  states: VecDeque<Result<S, S::Error>>,
}

impl<S: State> Bfs<S> {
  pub fn new(start: S) -> Self {
    let mut states = VecDeque::new();
    states.push_back(Ok(start));
    Self { states }
  }
}

impl<S: State> Iterator for Bfs<S> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop_front()?;
    let state = match result {
      Ok(state) => state,
      Err(_) => return Some(result),
    };

    let actions = state
      .actions()
      .into_iter()
      .map(|action| state.result(&action).map_err(|e| e.into()));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State> Search<S> for Bfs<S> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push_back(Ok(start));
    Ok(())
  }
}
