use super::{Search, State};
use crate::state::{DepthState, StateWrapper};

/// A Depth first traversal of possible states, up to a given depth.
///
/// As we give a limit to the depth of the search, we can avoid infinite loops.
pub struct Dls<S: State> {
  states: Vec<Result<DepthState<S>, S::Error>>,
  limit: usize,
}

impl<S: State> Dls<S> {
  pub fn new(start: S, limit: usize) -> Self {
    Self {
      states: vec![Ok(start.into())],
      limit,
    }
  }

  pub fn increment_limit(&mut self, start: S) -> Result<(), S::Error> {
    self.limit += 1;
    self.restart_from(start)?;
    Ok(())
  }
}

impl<S: State> Iterator for Dls<S> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop()?;
    let state = match result {
      Ok(state) => state,
      Err(e) => return Some(Err(e)),
    };

    if state.depth() >= self.limit {
      return Some(Ok(state.unwrap()));
    }

    let actions = state
      .actions()
      .into_iter()
      .map(|action| state.result(&action).map_err(S::Error::from));

    self.states.extend(actions);
    Some(Ok(state.unwrap()))
  }
}

impl<S: State> Search<S> for Dls<S> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(Ok(start.into()));
    Ok(())
  }
}
