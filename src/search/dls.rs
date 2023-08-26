use super::{Decision, Search, State};

pub struct Dls<S: State, D> {
  states: Vec<(Result<S, S::Error>, usize)>,
  actions_for: D,
  limit: usize,
}

impl<S: State, D> Dls<S, D> {
  pub fn new(start: S, actions_for: D, limit: usize) -> Self {
    Self {
      states: vec![(Ok(start), 0)],
      actions_for,
      limit,
    }
  }

  pub fn set_limit(&mut self, start: S, limit: usize)
  where
    D: Decision<S>,
  {
    self.limit = limit;
    self.restart_from(start);
  }
}

impl<S: State, D: Decision<S>> Iterator for Dls<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (result, depth) = self.states.pop()?;
    let state = match result {
      Ok(state) => state,
      Err(_) => return Some(result),
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
      .map(|action| (state.result(&action).map_err(S::Error::from), depth + 1));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for Dls<S, D> {
  fn restart_from(&mut self, start: S) {
    self.states.clear();
    self.states.push((Ok(start), 0));
  }
}
