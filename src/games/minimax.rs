//! The minimax algorithm.
//!
//! This attempts to find the best actions to take from the starting state,
//! assuming both players play optimally.

use crate::{prelude::SearchHeuristic, state::State};
use std::marker::PhantomData;

pub struct MiniMax<S, H> {
  pub state_value: H,
  pub depth: usize,
  pub players: usize,
  _state: PhantomData<S>,
}

impl<S, H> MiniMax<S, H> {
  pub fn new(state_value: H, depth: usize, players: usize) -> Self {
    Self {
      state_value,
      depth,
      players,
      _state: PhantomData,
    }
  }
}

impl<S: State, H: SearchHeuristic<S>> MiniMax<S, H>
where
  S::Action: Clone,
{
  pub fn minimax(&self, start: S) -> Option<(Vec<S::Action>, H::Cost)> {
    self.minimax_recursive(start, vec![])
  }

  fn minimax_recursive(
    &self,
    state: S,
    actions: Vec<S::Action>,
  ) -> Option<(Vec<S::Action>, H::Cost)> {
    let turn = actions.len();
    if turn == self.depth {
      let observed = state.observe().ok()?;
      let value = self.state_value.value(&observed);
      return Some((actions, value));
    }

    let paths = state.actions().into_iter().filter_map(|action| {
      let state = state.result(&action).ok()?;
      let mut actions = actions.clone();
      actions.push(action);
      self.minimax_recursive(state, actions)
    });

    if turn % self.players == 0 {
      paths.max_by_key(|(_, value)| value.clone())
    } else {
      paths.min_by_key(|(_, value)| value.clone())
    }
  }
}
