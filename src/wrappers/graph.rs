use super::{helpers::OptionIter, State, StateWrapper};
use dashmap::DashSet;
use derivative::Derivative;
use std::{fmt::Display, hash::Hash, rc::Rc};

/// A state that only expands if it has not been seen before.
///
/// This is useful for graphs that have cycles and commonly repeated states.
#[derive(Clone, Debug, Default, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GraphState<S: State>
where
  S::Observation: Hash + Eq,
{
  state: S,
  #[derivative(
    PartialEq = "ignore",
    PartialOrd = "ignore",
    Ord = "ignore",
    Hash = "ignore"
  )]
  seen: Rc<DashSet<S::Observation>>,
}

impl<S: State> From<S> for GraphState<S>
where
  S::Observation: Hash + Eq,
{
  fn from(state: S) -> Self {
    Self {
      state,
      seen: Rc::new(DashSet::new()),
    }
  }
}

impl<S: State + Display> Display for GraphState<S>
where
  S::Observation: Hash + Eq,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "GraphState:")?;
    write!(f, "{}", self.state)
  }
}

impl<S: State> State for GraphState<S>
where
  S::Observation: Hash + Eq + Clone,
{
  type Error = S::Error;
  type Observation = S::Observation;
  type Action = S::Action;
  type ActionIter = OptionIter<<S::ActionIter as IntoIterator>::IntoIter>;

  type ObserveError = S::ObserveError;
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    self.state.observe()
  }

  fn actions(&self) -> Self::ActionIter {
    // We only produce actions if we have not seen this state before
    if self.observe().map_or(false, |obs| self.seen.insert(obs)) {
      OptionIter::Some(self.state.actions().into_iter())
    } else {
      OptionIter::None
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

impl<S: State> StateWrapper<S> for GraphState<S>
where
  S::Observation: Hash + Eq,
{
  fn unwrap(self) -> S {
    self.state
  }

  fn replace(&mut self, state: S) -> S {
    std::mem::replace(&mut self.state, state)
  }
}
