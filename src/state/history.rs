use super::State;
use derivative::Derivative;

/// A state that tracks all actions taken.
///
/// This is useful for both debugging and explainability.
#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HistoryState<S: State> {
  state: S,
  #[derivative(
    PartialEq = "ignore",
    PartialOrd = "ignore",
    Ord = "ignore",
    Hash = "ignore"
  )]
  history: Vec<S::Action>,
}

impl<S: State> From<S> for HistoryState<S> {
  fn from(state: S) -> Self {
    Self {
      state,
      history: vec![],
    }
  }
}

impl<S: State> HistoryState<S> {
  pub fn state(self) -> S {
    self.state
  }
  pub fn history(&self) -> &[S::Action] {
    &self.history
  }
}

impl<S: State> State for HistoryState<S>
where
  S::Action: Clone,
{
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
    let mut history = self.history.clone();
    history.push(action.clone());
    self
      .state
      .result(action)
      .map(|state| HistoryState { state, history })
  }
}
