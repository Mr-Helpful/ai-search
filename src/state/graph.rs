use super::State;
use dashmap::DashSet;
use std::hash::Hash;

/// A state that only expands if it has not been seen before.
///
/// This is useful for graphs that have cycles and commonly repeated states.
pub struct GraphState<S: State> {
  state: S,
  seen: DashSet<S::Observation>,
}

impl<S: State> State for GraphState<S>
where
  S::Observation: Hash + Eq + Clone,
{
  type Error = S::Error;
  type Observation = S::Observation;
  type Action = S::Action;
  type ActionIter = Vec<Self::Action>;

  type ObserveError = S::ObserveError;
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    self.state.observe()
  }

  fn actions(&self) -> Self::ActionIter {
    // We only produce actions if we have not seen this state before
    if self.observe().map_or(false, |obs| self.seen.insert(obs)) {
      self.state.actions().into_iter().collect()
    } else {
      Vec::new()
    }
  }

  type ResultError = S::ResultError;
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    self.state.result(action).map(|state| GraphState {
      state,
      seen: self.seen.clone(),
    })
  }
}
