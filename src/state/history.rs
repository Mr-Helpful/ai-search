use super::{State, StateWrapper};
use derivative::Derivative;
use std::fmt::Display;

/// A state that tracks all actions taken.
///
/// This is useful for both debugging and explainability.
#[derive(Clone, Debug, Default, Derivative)]
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

impl<S: State + Display> Display for HistoryState<S> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let n_actns = self.history.len();
    let plural = if n_actns > 1 { "s" } else { "" };
    writeln!(f, "HistoryState ({} action{}):", n_actns, plural)?;
    write!(f, "{}", self.state)
  }
}

impl<S: State> HistoryState<S> {
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

impl<S: State> StateWrapper<S> for HistoryState<S> {
  fn unwrap_state(self) -> S {
    self.state
  }

  fn replace_state(&mut self, state: S) -> S {
    std::mem::replace(&mut self.state, state)
  }
}
