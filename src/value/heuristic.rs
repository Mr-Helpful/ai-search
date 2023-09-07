use super::State;
use std::ops::Add;

/// A heuristic function for a state.
///
/// This should be a guess at the path cost to the goal from the given state.
pub trait SearchHeuristic<S: State> {
  type Cost: Ord + Clone + Add<Output = Self::Cost>;
  fn value(&self, observed: &S::Observation) -> Self::Cost;
}

impl<S: State, C, F: Fn(&S::Observation) -> C> SearchHeuristic<S> for F
where
  C: Ord + Clone + Add<Output = C>,
{
  type Cost = C;
  fn value(&self, observed: &S::Observation) -> Self::Cost {
    self(observed)
  }
}
