use super::{Decision, Search, State, StateCost};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

pub struct Uniform<S: State, D, C: StateCost<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<S, Reverse<C::Cost>>,
  actions_for: D,
  node_cost: C,
}

impl<S, D, C> Iterator for Uniform<S, D, C>
where
  S: State + Hash + Eq,
  D: Decision<S>,
  C: StateCost<S>,
{
  type Item = Result<S, S::Error>;

  fn next(&mut self) -> Option<Self::Item> {
    let (state, Reverse(path_cost)) = self.states.pop()?;
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
        let cost = self.node_cost.cost(&state);
        (state, Reverse(path_cost.clone() + cost))
      });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>, C: StateCost<S>> Search<S> for Uniform<S, D, C>
where
  S: Hash + Eq,
{
  fn restart_from(&mut self, start: S) {
    self.states.clear();
    let cost = self.node_cost.cost(&start);
    self.states.push(start, Reverse(cost));
  }
}