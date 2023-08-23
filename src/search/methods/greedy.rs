use super::{Decision, Search, State, StateCost};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

pub struct Greedy<S, D, H: StateCost<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<S, Reverse<H::Cost>>,
  actions_for: D,
  heuristic: H,
}

impl<S, D, H> Iterator for Greedy<S, D, H>
where
  S: State + Hash + Eq,
  D: Decision<S>,
  H: StateCost<S>,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, Reverse(_)) = self.states.pop()?;
    let observation = match state.observe() {
      Ok(observation) => observation,
      Err(e) => return Some(Err(e.into())),
    };

    let actions = self
      .actions_for
      .actions(observation)
      .into_iter()
      .filter_map(|action| state.result(&action).ok())
      .map(|state| {
        let cost = self.heuristic.cost(&state);
        (state, Reverse(cost))
      });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>, C: StateCost<S>> Search<S> for Greedy<S, D, C> where S: Hash + Eq {}
