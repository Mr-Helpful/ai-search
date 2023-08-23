use crate::state::State;

pub mod bfs;
pub mod dfs;
pub mod dls;
pub mod uniform;

pub trait Search<S: State>: Iterator<Item = Result<S, S::Error>> + Sized {
  /// Returns the next valid state generated by this search
  fn next_valid(&mut self) -> Option<S> {
    self.find_map(|item| item.ok())
  }

  /// Returns the next goal state generated by this search.
  fn next_goal(&mut self, goal: impl Copy + Fn(&S::Observation) -> bool) -> Option<S> {
    self
      .filter_map(|item| item.ok())
      .find(|state| state.observe().map_or(false, goal))
  }
}

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

mod cost;
use cost::StateCost;
