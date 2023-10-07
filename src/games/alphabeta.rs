use crate::prelude::{SearchHeuristic, State};
use std::{
  fmt::Debug,
  marker::PhantomData,
  ops::{Bound, RangeBounds},
};

fn ge_bound<O: Ord>(bound: &Bound<O>, item: &O) -> bool {
  match bound {
    Bound::Unbounded => true,
    Bound::Included(bound) => bound <= item,
    Bound::Excluded(bound) => bound < item,
  }
}

fn le_bound<O: Ord>(bound: &Bound<O>, item: &O) -> bool {
  match bound {
    Bound::Unbounded => true,
    Bound::Included(bound) => bound >= item,
    Bound::Excluded(bound) => bound > item,
  }
}

fn included<O>(bound: Bound<O>) -> Option<O> {
  match bound {
    Bound::Included(item) => Some(item),
    Bound::Excluded(_) => None,
    Bound::Unbounded => None,
  }
}

pub struct AlphaBeta<S, H> {
  pub state_value: H,
  pub depth: usize,
  pub players: usize,
  _state: PhantomData<S>,
}

impl<S, H> AlphaBeta<S, H> {
  pub fn new(state_value: H, depth: usize, players: usize) -> Self {
    Self {
      state_value,
      depth,
      players,
      _state: PhantomData,
    }
  }
}

impl<S: State, H: SearchHeuristic<S>> AlphaBeta<S, H>
where
  S::Action: Clone,
{
  pub fn alphabeta(&self, start: S) -> Option<(Vec<S::Action>, H::Cost)> {
    self.alphabeta_limits(start, ..)
  }

  pub fn alphabeta_limits<R: RangeBounds<H::Cost>>(
    &self,
    start: S,
    alphabeta: R,
  ) -> Option<(Vec<S::Action>, H::Cost)> {
    self.alphabeta_recursive(
      start,
      vec![],
      (
        alphabeta.start_bound().cloned(),
        alphabeta.end_bound().cloned(),
      ),
    )
  }

  fn alphabeta_recursive(
    &self,
    state: S,
    actions: Vec<S::Action>,
    (alpha, beta): (Bound<H::Cost>, Bound<H::Cost>),
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
      Some((actions, state))
    });

    if turn % self.players == 0 {
      self.alphabeta_max(paths, (alpha, beta), actions.clone())
    } else {
      self.alphabeta_min(paths, (alpha, beta), actions.clone())
    }
  }

  fn alphabeta_min(
    &self,
    paths: impl Iterator<Item = (Vec<S::Action>, S)>,
    (alpha, mut beta): (Bound<H::Cost>, Bound<H::Cost>),
    actions: Vec<S::Action>,
  ) -> Option<(Vec<S::Action>, H::Cost)> {
    for (actions, state) in paths {
      if let Some((actions, value)) =
        self.alphabeta_recursive(state, actions, (alpha.clone(), beta.clone()))
      {
        if le_bound(&alpha, &value) {
          return Some((actions, value));
        } else if le_bound(&beta, &value) {
          beta = Bound::Included(value);
        }
      }
    }
    included(beta).map(|value| (actions, value))
  }

  fn alphabeta_max(
    &self,
    paths: impl Iterator<Item = (Vec<S::Action>, S)>,
    (mut alpha, beta): (Bound<H::Cost>, Bound<H::Cost>),
    actions: Vec<S::Action>,
  ) -> Option<(Vec<S::Action>, H::Cost)> {
    for (actions, state) in paths {
      if let Some((actions, value)) =
        self.alphabeta_recursive(state, actions, (alpha.clone(), beta.clone()))
      {
        if ge_bound(&beta, &value) {
          return Some((actions, value));
        } else if ge_bound(&alpha, &value) {
          alpha = Bound::Included(value);
        }
      }
    }
    included(alpha).map(|value| (actions, value))
  }
}
