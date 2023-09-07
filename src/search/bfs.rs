use super::{Decision, Search, State};
use std::collections::VecDeque;

/// A Breadth first traversal of possible states.
///
/// Will only explore states at a given depth after all states at the previous
/// depth have been explored.
pub struct Bfs<S: State, D> {
  states: VecDeque<Result<S, S::Error>>,
  actions_for: D,
}

impl<S: State, D: Decision<S>> Iterator for Bfs<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop_front()?;
    let state = match result {
      Ok(state) => state,
      Err(_) => return Some(result),
    };

    let observation = match state.observe() {
      Ok(observation) => observation,
      Err(e) => return Some(Err(e.into())),
    };

    let actions = self
      .actions_for
      .actions(&observation)
      .into_iter()
      .map(|action| state.result(&action).map_err(|e| e.into()));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for Bfs<S, D> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push_back(Ok(start));
    Ok(())
  }
}
