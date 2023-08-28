use super::{Decision, Dls, Search, State};

pub struct Ids<S: State, D> {
  search: Dls<S, D>,
  start: S,
}

impl<S: State, D: Decision<S>> Iterator for Ids<S, D>
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

impl<S: State, D: Decision<S>> Search<S> for Ids<S, D>
where
  S: Clone,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.search.restart_from(start.clone())?;
    Ok(())
  }
}
