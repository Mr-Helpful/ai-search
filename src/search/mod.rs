use super::{Decision, State, StateCost};

mod bfs;
mod dfs;
mod dls;
mod greedy;
mod uniform;
pub use bfs::BFS;
pub use dfs::DFS;
pub use dls::DLS;
pub use greedy::Greedy;
pub use uniform::Uniform;

pub trait Search<S: State>: Iterator<Item = Result<S, S::Error>> + Sized {
  /// Restarts the search from the given state
  fn restart_from(&mut self, start: S);

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
