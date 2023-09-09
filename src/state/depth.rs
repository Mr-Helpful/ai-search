use super::State;
use derivative::Derivative;

/// A state that tracks its depth.
///
/// This is useful for:
/// - Depth limited search
/// - Games (where depth is used to determine the player)
#[derive(Clone, Copy, Debug, Default, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DepthState<S> {
  state: S,
  #[derivative(
    PartialEq = "ignore",
    PartialOrd = "ignore",
    Ord = "ignore",
    Hash = "ignore"
  )]
  depth: usize,
}

impl<S> From<S> for DepthState<S> {
  fn from(state: S) -> Self {
    Self { state, depth: 0 }
  }
}

impl<S> DepthState<S> {
  pub fn state(self) -> S {
    self.state
  }
  pub fn depth(&self) -> usize {
    self.depth
  }
}

impl<S: State> State for DepthState<S> {
  type Error = S::Error;
  type Observation = S::Observation;
  type Action = S::Action;
  type ActionIter = S::ActionIter;
  type ObserveError = S::ObserveError;
  type ResultError = S::ResultError;

  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    self.state.observe()
  }

  fn actions(&self) -> Self::ActionIter {
    self.state.actions()
  }

  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    self.state.result(action).map(|state| DepthState {
      state,
      depth: self.depth + 1,
    })
  }
}
