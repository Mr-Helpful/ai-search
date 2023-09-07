use super::{Decision, Search, State};

/// A Depth first traversal of possible states.
///
/// Will explore paths of states until a dead end is reached, then back track.
pub struct Dfs<S: State, D> {
  states: Vec<Result<S, S::Error>>,
  actions_for: D,
}

impl<S: State, D> Dfs<S, D> {
  pub fn new(start: S, actions_for: D) -> Self {
    Self {
      states: vec![Ok(start)],
      actions_for,
    }
  }
}

impl<S: State, D: Decision<S>> Iterator for Dfs<S, D> {
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
      .actions(&observation)
      .into_iter()
      .map(|action| state.result(&action).map_err(|e| e.into()));

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>> Search<S> for Dfs<S, D> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(Ok(start));
    Ok(())
  }
}
