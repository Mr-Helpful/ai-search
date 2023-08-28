use super::State;

pub struct DepthState<S> {
  state: S,
  depth: usize,
}

impl<S: State> State for DepthState<S> {
  type Error = S::Error;
  type Observation = (S::Observation, usize);
  type Action = S::Action;
  type ObserveError = S::ObserveError;
  type ResultError = S::ResultError;

  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    Ok((self.state.observe()?, self.depth))
  }

  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    self.state.result(action).map(|state| DepthState {
      state,
      depth: self.depth + 1,
    })
  }
}
