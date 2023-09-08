use crate::prelude::SearchCost;

use super::State;
;

/// A state that tracks the path cost to the current state.
pub struct PathCostState<S: State, C: SearchCost<S>> {
  state: S,
  actn_cost: C,
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

  pub fn state(self) -> S {
    self.state
  }
}

impl<S: State, C: SearchCost<S> + Clone> State for PathCostState<S, C> {
  type Error = S::Error;
  type Observation = (S::Observation, C::Cost);
  type Action = S::Action;
  type ActionIter = S::ActionIter;
  type ObserveError = S::ObserveError;
  type ResultError = S::ResultError;

  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    Ok((self.state.observe()?, self.path_cost.clone()))
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
