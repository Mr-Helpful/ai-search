use super::State;
use dashmap::DashSet;
use std::hash::Hash;

pub trait Decision<S: State> {
  type Actions: IntoIterator<Item = S::Action>;
  fn actions(&self, observation: &S::Observation) -> Self::Actions;
}

impl<F: Fn(&S::Observation) -> A, S: State, A: IntoIterator<Item = S::Action>> Decision<S> for F {
  type Actions = A;
  fn actions(&self, observation: &S::Observation) -> Self::Actions {
    self(observation)
  }
}

#[derive(Debug, Clone, Default)]
pub struct GraphDecision<S: State, D: Decision<S>>
where
  S::Observation: Hash + Eq,
{
  seen: DashSet<S::Observation>,
  actions_for: D,
}

impl<S: State, D: Decision<S>> From<D> for GraphDecision<S, D>
where
  S::Observation: Hash + Eq,
{
  fn from(actions_for: D) -> Self {
    Self {
      seen: DashSet::new(),
      actions_for,
    }
  }
}

impl<S: State, D: Decision<S>> Decision<S> for GraphDecision<S, D>
where
  S::Observation: Hash + Eq + Clone,
{
  type Actions = Vec<S::Action>;
  fn actions(&self, observation: &<S as State>::Observation) -> Self::Actions {
    if !self.seen.insert(observation.clone()) {
      return Vec::new();
    }
    self.actions_for.actions(observation).into_iter().collect()
  }
}
