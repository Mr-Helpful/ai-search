use super::{Decision, Search, State};
use crate::state::DepthState;

/// A Depth first traversal of possible states, up to a given depth.
///
/// As we give a limit to the depth of the search, we can avoid infinite loops.
pub struct Dls<S: State, D> {
  states: Vec<Result<DepthState<S>, S::Error>>,
  actions_for: D,
  limit: usize,
}

impl<S: State, D> Dls<S, D> {
  pub fn new(start: S, actions_for: D, limit: usize) -> Self {
    Self {
      states: vec![Ok(start.into())],
      actions_for,
      limit,
    }
  }

  pub fn increment_limit(&mut self, start: S) -> Result<(), S::Error>
  where
    D: Decision<S>,
  {
    self.limit += 1;
    self.restart_from(start)?;
    Ok(())
  }
}

impl<S: State, D: Decision<S>> Iterator for Dls<S, D> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop()?;
    let state = match result {
      Ok(state) => state,
      Err(e) => return Some(Err(e)),
    };

    let (observation, depth) = match state.observe() {
      Ok(observation) => observation,
      Err(e) => return Some(Err(e.into())),
    };

    if depth >= self.limit {
      return Some(Ok(state.state()));
    }

    let actions = self
      .actions_for
      .actions(&observation)
      .into_iter()
      .map(|action| state.result(&action).map_err(S::Error::from));

    self.states.extend(actions);
    Some(Ok(state.state()))
  }
}

impl<S: State, D: Decision<S>> Search<S> for Dls<S, D> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(Ok(start.into()));
    Ok(())
  }
}
