use super::{Dls, Search, State};

/// An Iterative deepening search.
///
/// This search will perform a depth first search up to a given depth, then
/// restart with a deeper depth. This will continue until a goal state is found.
pub struct Ids<S: State> {
  search: Dls<S>,
  start: S,
}

impl<S: State> Ids<S>
where
  S: Clone,
{
  pub fn new(start: S) -> Self {
    Self {
      search: Dls::new(start.clone(), 0),
      start,
    }
  }
}

impl<S: State> Iterator for Ids<S>
where
  S: Clone,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let result = self.search.next();
      if result.is_some() {
        return result;
      }

      let set_result = self.search.increment_limit(self.start.clone());
      if let Err(state_error) = set_result {
        return Some(Err(state_error));
      }
    }
  }
}

impl<S: State> Search<S> for Ids<S>
where
  S: Clone,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.search.restart_from(start.clone())?;
    Ok(())
  }
}
