use super::{Decision, Dls, Search, State};

pub struct Idfs<S: State, D> {
  start: S,
  limit: usize,
  search: Dls<S, D>,
}

impl<S: State, D: Decision<S>> Iterator for Idfs<S, D>
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

      self.limit += 1;
      self.search.set_limit(self.start.clone(), self.limit);
    }
  }
}

impl<S: State, D: Decision<S>> Search<S> for Idfs<S, D>
where
  S: Clone,
{
  fn restart_from(&mut self, start: S) {
    self.start = start;
    self.limit = 0;
    self.search.set_limit(self.start.clone(), self.limit);
  }
}
