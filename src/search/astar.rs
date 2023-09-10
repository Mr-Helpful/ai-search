use super::{Search, State};
use crate::{
  value::{SearchCost, SearchHeuristic},
  wrappers::{PathCostState, StateWrapper},
};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash, ops::Add};

/// A Greedy traversal of possible states.
///
/// Will explore states with the lowest heuristic value first.
pub struct Astar<S: State, C: SearchCost<S>, H: SearchHeuristic<S>>
where
  S: Hash + Eq,
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  states: PriorityQueue<PathCostState<S, C>, Reverse<<C::Cost as Add<H::Cost>>::Output>>,
  action_cost: C,
  heuristic: H,
}

impl<S: State, C: SearchCost<S> + Clone, H: SearchHeuristic<S>> Astar<S, C, H>
where
  S: Hash + Eq,
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  pub fn new(start: S, action_cost: C, heuristic: H) -> Self {
    let mut states = PriorityQueue::new();
    let obs = start.observe().map_err(S::Error::from).unwrap();
    let cost = heuristic.value(&obs);
    states.push(
      PathCostState::new(start, action_cost.clone()),
      Reverse(<C::Cost as Add<H::Cost>>::add(Default::default(), cost)),
    );
    Self {
      states,
      action_cost,
      heuristic,
    }
  }
}

impl<S: State, C: SearchCost<S>, H: SearchHeuristic<S>> Iterator for Astar<S, C, H>
where
  S: Hash + Eq,
  C: Clone,
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, _) = self.states.pop()?;

    let actions = state.actions().into_iter().filter_map(|action| {
      let new_state = state.result(&action).ok()?;
      let state_value = self.heuristic.value(&new_state.observe().ok()?);
      let state_astar = <C::Cost as Add<H::Cost>>::add(new_state.path_cost(), state_value);
      Some((new_state, Reverse(state_astar)))
    });

    self.states.extend(actions);
    Some(Ok(state.unwrap()))
  }
}

impl<S: State, C: SearchCost<S>, H: SearchHeuristic<S>> Search<S> for Astar<S, C, H>
where
  S: Hash + Eq,
  C: Clone,
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    let cost = self.heuristic.value(&start.observe()?);
    self.states.push(
      PathCostState::new(start, self.action_cost.clone()),
      Reverse(<C::Cost as Add<H::Cost>>::add(Default::default(), cost)),
    );
    Ok(())
  }
}
