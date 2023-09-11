//! The minimax algorithm.
//!
//! This attempts to find the best actions to take from the starting state,
//! assuming both players play optimally.
//!
//! @todo Implement MVP 2-Player minimax.<br>
//! @todo Modify for N-Player minimax.<br>
//! @todo Add Alpha-Beta pruning.<br>
use crate::state::State;

pub struct MiniMax<S: State> {
  states: Vec<S>,
}
