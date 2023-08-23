use super::State;
use std::collections::HashSet;
use std::hash::Hash;

pub trait Decision<S: State> {
  type Actions: IntoIterator<Item = S::Action>;
  fn actions(&mut self, observation: &S::Observation) -> Self::Actions;
}

impl<F: Fn(&S::Observation) -> A, S: State, A: IntoIterator<Item = S::Action>> Decision<S> for F {
  type Actions = A;
  fn actions(&mut self, observation: &S::Observation) -> Self::Actions {
    self(observation)
  }
}

pub struct GraphDecision<S: State, D: Decision<S>> {
  seen: HashSet<S::Observation>,
  actions_for: D,
}

impl<S: State, D: Decision<S>> Decision<S> for GraphDecision<S, D>
where
  S::Observation: Hash + Eq + Clone,
{
  type Actions = Vec<S::Action>;
  fn actions(&mut self, observation: &<S as State>::Observation) -> Self::Actions {
    if self.seen.contains(observation) {
      return Vec::new();
    }

    self.seen.insert(observation.clone());
    self.actions_for.actions(observation).into_iter().collect()
  }
}
