use super::{Search, State};
use crate::value::SearchCost;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

/// A Uniform cost traversal of possible states.
///
/// Will explore states with the lowest accumulated path cost first.
pub struct Uniform<S: State, C: SearchCost<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<S, Reverse<Option<C::Cost>>>,
  action_cost: C,
}

impl<S: State, C: SearchCost<S>> Uniform<S, C>
where
  S: Hash + Eq,
{
  pub fn new(start: S, action_cost: C) -> Self {
    let mut states = PriorityQueue::new();
    states.push(start, Reverse(None));
    Self {
      states,
      action_cost,
    }
  }
}

impl<S: State, C: SearchCost<S>> Iterator for Uniform<S, C>
where
  S: Hash + Eq,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, Reverse(path_cost)) = self.states.pop()?;

    let actions = state.actions().into_iter().filter_map(|action| {
      let new_state = state.result(&action).ok()?;
      let actn_cost = self.action_cost.cost(&action);
      let cost = path_cost
        .clone()
        .map_or(actn_cost.clone(), |c| c + actn_cost);
      Some((new_state, Reverse(Some(cost))))
    });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, C: SearchCost<S>> Search<S> for Uniform<S, C>
where
  S: Hash + Eq,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(start, Reverse(None));
    Ok(())
  }
}
