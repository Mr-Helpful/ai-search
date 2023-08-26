// use super::{Decision, State, DLS};

// pub struct IDFS<S: State, D> {
//   start: S,
//   depth: usize,
//   search: DLS<S, D>,
// }

// impl<S: State, D: Decision<S>> Iterator for IDFS<S, D>
// where
//   S: Clone,
// {
//   type Item = Result<S, S::Error>;

//   fn next(&mut self) -> Option<Self::Item> {
//     let result = self.search.next()?;
//     if result.is_err() {
//       self.search.limit += 1;
//     }
//     Some(result)
//   }
// }
