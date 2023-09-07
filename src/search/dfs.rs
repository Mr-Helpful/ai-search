use super::{Search, State};

/// A Depth first traversal of possible states.
///
/// Will explore paths of states until a dead end is reached, then back track.
pub struct Dfs<S: State> {
  states: Vec<Result<S, S::Error>>,
}

impl<S: State> Dfs<S> {
  pub fn new(start: S) -> Self {
    Self {
      states: vec![Ok(start)],
    }
  }
}

impl<S: State> Iterator for Dfs<S> {
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let result = self.states.pop()?;
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

impl<S: State> Search<S> for Dfs<S> {
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(Ok(start));
    Ok(())
  }
}
