use crate::state::State;

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
