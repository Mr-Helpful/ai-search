//! Algorithms for searching a state space.
//!
//! We define a search algorithm as an iterator over a state space, with the
//! possibility of returning an `Err` when a state fails to expand.

use crate::state::State;

mod astar;
mod bfs;
mod dfs;
mod dls;
mod greedy;
mod ids;
mod uniform;
pub use astar::Astar;
pub use bfs::Bfs;
pub use dfs::Dfs;
pub use dls::Dls;
pub use greedy::Greedy;
pub use ids::Ids;
pub use uniform::Uniform;

/// A generic search algorithm should act as an traversal over some tree of
/// states, with the possiblity of failure upon expanding each state.
/// A search algorithm should also be able to restart from a given state.
/// Using the implemented traversal, we can provide implementations for the
/// next valid and goal state.
pub trait Search<S: State>: Iterator<Item = Result<S, S::Error>> + Sized {
  /// Restarts the search from the given state
  fn restart_from(&mut self, start: S) -> Result<(), S::Error>;

  /// Returns the next valid state generated by this search
  fn next_valid(&mut self) -> Option<S> {
    self.find_map(|item| item.ok())
  }

  /// Returns the next goal state generated by this search.
  fn next_goal(&mut self, goal: impl Fn(&S::Observation) -> bool) -> Option<S> {
    self
      .filter_map(|item| item.ok())
      .find(|state| state.observe().as_ref().map_or(false, &goal))
  }
}
