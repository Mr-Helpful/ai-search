use super::State;
use std::ops::Add;

pub trait SearchCost<S: State> {
  type Cost: Ord + Clone + Add<Output = Self::Cost>;
  fn cost(&self, action: &S::Action) -> Self::Cost;
}

impl<S: State, C, F: Fn(&S::Action) -> C> SearchCost<S> for F
where
  C: Ord + Clone + Add<Output = C>,
{
  type Cost = C;
  fn cost(&self, action: &S::Action) -> Self::Cost {
    self(action)
  }
}
