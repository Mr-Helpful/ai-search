use super::{Decision, Search, State};

pub struct DLS<S: State, D> {
  states: Vec<(Result<S, S::ResultError>, usize)>,
  actions_for: D,
  limit: usize,
}

impl<S: State, D: Decision<S>> Iterator for DLS<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (result, depth) = self.states.pop()?;
    let state = match result {
      Ok(state) => state,
      Err(e) => return Some(Err(e.into())),
    };

    if depth >= self.limit {
      return Some(Ok(state));
    }

    let observation = match state.observe() {
      Ok(observation) => observation,
      Err(e) => return Some(Err(e.into())),
    };

    let actions = self
      .actions_for
      .actions(observation)
      .into_iter()
      .map(|action| (state.result(&action), depth + 1));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for DLS<S, D> {
  fn restart_from(&mut self, start: S) {
    self.states.clear();
    self.states.push((Ok(start), 0));
  }
}
