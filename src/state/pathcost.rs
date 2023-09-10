use super::{State, StateWrapper};
use crate::prelude::SearchCost;
use derivative::Derivative;
use std::fmt::Display;

/// A state that tracks the path cost to the current state.
#[derive(Clone, Debug, Default, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathCostState<S: State, C: SearchCost<S>> {
  state: S,
  #[derivative(
    PartialEq = "ignore",
    PartialOrd = "ignore",
    Ord = "ignore",
    Hash = "ignore"
  )]
  actn_cost: C,
  #[derivative(
    PartialEq = "ignore",
    PartialOrd = "ignore",
    Ord = "ignore",
    Hash = "ignore"
  )]
  path_cost: C::Cost,
}

impl<S: State, C: SearchCost<S>> PathCostState<S, C> {
  pub fn new(state: S, actn_cost: C) -> Self {
    Self {
      state,
      actn_cost,
      path_cost: Default::default(),
    }
  }
}

impl<S: State + Display, C: SearchCost<S>> Display for PathCostState<S, C>
where
  C::Cost: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    writeln!(f, "PathCostState (path cost {}):", self.path_cost)?;
    write!(f, "{}", self.state)
  }
}

impl<S: State, C: SearchCost<S>> PathCostState<S, C> {
  pub fn path_cost(&self) -> C::Cost {
    self.path_cost.clone()
  }
}

impl<S: State, C: SearchCost<S> + Clone> State for PathCostState<S, C> {
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
    self.state.result(action).map(|state| PathCostState {
      state,
      actn_cost: self.actn_cost.clone(),
      path_cost: self.path_cost.clone() + self.actn_cost.cost(action),
    })
  }
}

impl<S: State, C: SearchCost<S> + Clone> StateWrapper<S> for PathCostState<S, C> {
  fn unwrap(self) -> S {
    self.state
  }

  fn replace(&mut self, state: S) -> S {
    std::mem::replace(&mut self.state, state)
  }
}
