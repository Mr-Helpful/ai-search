use super::{Decision, Search, State, StateCost};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

pub struct Uniform<S: State, D, C: StateCost<S>>
where
  S: Hash + Eq,
{
  states: PriorityQueue<S, Reverse<Option<C::Cost>>>,
  actions_for: D,
  action_cost: C,
}

impl<S: State, D, C: StateCost<S>> Iterator for Uniform<S, D, C>
where
  S: Hash + Eq,
  D: Decision<S>,
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
      .filter_map(|action| {
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

impl<S: State, D: Decision<S>, C: StateCost<S>> Search<S> for Uniform<S, D, C>
where
  S: Hash + Eq,
{
  fn restart_from(&mut self, start: S) -> Result<(), S::Error> {
    self.states.clear();
    self.states.push(start, Reverse(None));
    Ok(())
  }
}
