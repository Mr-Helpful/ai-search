use super::{Search, State};
use crate::value::{SearchCost, SearchHeuristic};
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
  states: PriorityQueue<(S, C::Cost), Reverse<<C::Cost as Add<H::Cost>>::Output>>,
  action_cost: C,
  heuristic: H,
}

impl<S: State, C: SearchCost<S>, H: SearchHeuristic<S>> Astar<S, C, H>
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
      (start, Default::default()),
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
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let ((state, path_cost), _) = self.states.pop()?;

    let actions = state.actions().into_iter().filter_map(|action| {
      let new_state = state.result(&action).ok()?;
      let actn_cost = self.action_cost.cost(&action);
      let state_value = self.heuristic.value(&new_state.observe().ok()?);
      let new_path_cost = path_cost.clone() + actn_cost;
      Some((
        (new_state, new_path_cost.clone()),
        Reverse(<C::Cost as Add<H::Cost>>::add(new_path_cost, state_value)),
      ))
    });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, C: SearchCost<S>, H: SearchHeuristic<S>> Search<S> for Astar<S, C, H>
where
  S: Hash + Eq,
  C::Cost: Hash + Add<H::Cost>,
  <C::Cost as Add<H::Cost>>::Output: Ord,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    let cost = self.heuristic.value(&start.observe()?);
    self.states.push(
      (start, Default::default()),
      Reverse(<C::Cost as Add<H::Cost>>::add(Default::default(), cost)),
    );
    Ok(())
  }
}
