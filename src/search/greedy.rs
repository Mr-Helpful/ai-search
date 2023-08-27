use super::{Decision, Search, State, StateHeuristic};
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, hash::Hash};

pub struct Greedy<S: State, D, H: StateHeuristic<S>>
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
  H: StateHeuristic<S>,
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
      .actions(&observation)
      .into_iter()
      .filter_map(|action| {
        let new_state = state.result(&action).ok()?;
        let value = self.heuristic.value(&new_state.observe().ok()?);
        Some((new_state, Reverse(value)))
      });

    self.states.extend(actions);
    Some(Ok(state))
  }
}

impl<S: State, D: Decision<S>, C: StateHeuristic<S>> Search<S> for Greedy<S, D, C>
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
