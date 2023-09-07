use super::{Search, State};
use crate::value::SearchHeuristic;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

/// A Greedy traversal of possible states.
///
/// Will explore states with the lowest heuristic value first.
pub struct Greedy<S: State, H: SearchHeuristic<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<S, Reverse<H::Cost>>,
  heuristic: H,
}

impl<S: State, H: SearchHeuristic<S>> Greedy<S, H>
where
  S: Hash + Eq,
{
  pub fn new(start: S, heuristic: H) -> Self {
    let mut states = PriorityQueue::new();
    let obs = start.observe().map_err(S::Error::from).unwrap();
    let cost = heuristic.value(&obs);
    states.push(start, Reverse(cost));
    Self { states, heuristic }
  }
}

impl<S, H> Iterator for Greedy<S, H>
where
  S: State + Hash + Eq,
  H: SearchHeuristic<S>,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, Reverse(_)) = self.states.pop()?;

    let actions = state.actions().into_iter().filter_map(|action| {
      let new_state = state.result(&action).ok()?;
      let value = self.heuristic.value(&new_state.observe().ok()?);
      Some((new_state, Reverse(value)))
    });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, C: SearchHeuristic<S>> Search<S> for Greedy<S, C>
where
  S: Hash + Eq,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    let cost = self.heuristic.value(&start.observe()?);
    self.states.push(start, Reverse(cost));
    Ok(())
  }
}
