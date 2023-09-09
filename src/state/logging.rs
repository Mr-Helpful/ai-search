use super::{State, StateWrapper};
use std::fmt::Display;

/// A state that tracks all actions taken.
///
/// This is useful for both debugging and explainability.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoggingState<S: State> {
  state: S,
}

impl<S: State> From<S> for LoggingState<S> {
  fn from(state: S) -> Self {
    Self { state }
  }
}

impl<S: State + Display> Display for LoggingState<S> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "LoggingState:")?;
    write!(f, "{}", self.state)
  }
}

impl<S: State> State for LoggingState<S>
where
  S: Display,
  S::Action: Clone + Display,
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
    println!("Transitioning to Next State");
    println!("> Action: {}", action);
    self
      .state
      .result(action)
      .map(|state| {
        println!("> State:\n{}", state);
        LoggingState { state }
      })
      .map_err(|e| {
        println!("> Failed!");
        e
      })
  }
}

impl<S: State> StateWrapper<S> for LoggingState<S> {
  fn unwrap(self) -> S {
    self.state
  }

  fn replace(&mut self, state: S) -> S {
    std::mem::replace(&mut self.state, state)
  }
}
