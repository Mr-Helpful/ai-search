use super::{Decision, Search, State};

pub struct DFS<S: State, D> {
  states: Vec<Result<S, S::Error>>,
  actions_for: D,
}

impl<S: State, D: Decision<S>> Iterator for DFS<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop()?;
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
      .actions(observation)
      .into_iter()
      .map(|action| state.result(&action).map_err(|e| e.into()));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for DFS<S, D> {}
