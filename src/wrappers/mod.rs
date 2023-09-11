use crate::state::State;

mod helpers;

mod depth;
pub use depth::DepthState;
mod graph;
pub use graph::GraphState;
mod pathcost;
pub use pathcost::PathCostState;
mod history;
pub use history::HistoryState;
mod logging;
pub use logging::LoggingState;

/// A wrapper for states that allows for the state to be extracted
pub trait StateWrapper<S: State> {
  /// Extracts the state from the wrapper
  fn unwrap(self) -> S;

  /// Replaces the state in the wrapper with a new state
  fn replace(&mut self, state: S) -> S;
}
