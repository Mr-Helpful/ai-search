use super::{Decision, Search, State};
use std::collections::VecDeque;

pub struct BFS<S: State, D> {
  states: VecDeque<Result<S, S::ResultError>>,
  actions_for: D,
}

impl<S: State, D: Decision<S>> Iterator for BFS<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop_front()?;
    let state = match result {
      Ok(state) => state,
      Err(e) => return Some(Err(e.into())),
    };

    let observation = match state.observe() {
      Ok(observation) => observation,
      Err(e) => return Some(Err(e.into())),
    };

    let actions = self
      .actions_for
      .actions(observation)
      .into_iter()
      .map(|action| state.result(&action));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for BFS<S, D> {}
