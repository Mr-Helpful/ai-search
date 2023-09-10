use super::{Search, State};
use crate::{
  state::{PathCostState, StateWrapper},
  value::SearchCost,
};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

/// A Uniform cost traversal of possible states.
///
/// Will explore states with the lowest accumulated path cost first.
pub struct Uniform<S: State, C: SearchCost<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<PathCostState<S, C>, Reverse<C::Cost>>,
  action_cost: C,
}

impl<S: State, C: SearchCost<S>> Uniform<S, C>
where
  S: Hash + Eq,
  C: Clone,
{
  pub fn new(start: S, action_cost: C) -> Self {
    let mut states = PriorityQueue::new();
    states.push(
      PathCostState::new(start, action_cost.clone()),
      Reverse(Default::default()),
    );
    Self {
      states,
      action_cost,
    }
  }
}

impl<S: State, C: SearchCost<S>> Iterator for Uniform<S, C>
where
  S: Hash + Eq,
  C: Clone,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, _) = self.states.pop()?;

    let actions = state.actions().into_iter().filter_map(|action| {
      let new_state = state.result(&action).ok()?;
      let path_cost = new_state.path_cost();
      Some((new_state, Reverse(path_cost)))
    });

    self.states.extend(actions);
    Some(Ok(state.unwrap()))
  }
}

impl<S: State, C: SearchCost<S>> Search<S> for Uniform<S, C>
where
  S: Hash + Eq,
  C: Clone,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(
      PathCostState::new(start, self.action_cost.clone()),
      Reverse(Default::default()),
    );
    Ok(())
  }
}
