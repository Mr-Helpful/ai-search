//! A generic implementation of state for search methods.
//!
//! The `State` trait is used in all search methods defined in this library and
//! therefore anything implementing the `State` trait can be searched over.
//!
//! Honestly, this is probably too generic, as it allows for both limiting the
//! set of actions that can be taken on a state, and possibly erroring when
//! attempting to transition to a new state.
//!
//! Also, literally none of the states I've implemented so far actually produce
//! errors for any of the methods that they implement, so the error handling
//! may be straight up unneeded.

use std::fmt::Debug;

/// A generic implementation of state for search methods.
///
/// We make decisions on the state based on observations of the state, to allow
/// for environments that are not fully observable (i.e. a poker game where we
/// cannot see the other players cards).
///
/// States can be thought of as similar to nodes on a graph, with the actions
/// representing edges. We also look to solve the slightly reduced problem of
/// finding any path to a goal state, rather than the shortest path.
pub trait State: Sized {
  /// A supertype for all errors thrown during state generation
  ///
  /// All errors should be convertable into this type
  type Error: Debug + From<Self::ObserveError> + From<Self::ResultError>;

  /// A type for observations of the state.
  ///
  /// Observations should be used to inform the order in which we search the
  /// available space, primarily through ordering states by their heuristic
  /// value.
  ///
  /// An Observation should be a (potentially) restricted view on the state
  type Observation;

  /// The error produced whilst attempting to observe the state
  type ObserveError;
  /// Returns an observation of the state
  ///
  /// This should be used with to inform which action to take next on this state
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError>;

  /// A type for actions that can be taken on this state
  ///
  /// We should be able to transition from this state with anything satisfying
  /// the action type
  type Action;

  /// A type for collections of Actions for this state
  ///
  /// This represents all valid Actions that can be taken on this state
  type ActionIter: IntoIterator<Item = Self::Action>;

  /// Returns all valid actions that can be taken on this state
  ///
  /// This should be used with `observe` to inform the next action to take
  fn actions(&self) -> Self::ActionIter;

  /// The error produced whilst attempting to transition to a new state
  type ResultError;
  /// Takes an action and transitions into a new state
  ///
  /// This should produce a new state from a reference to this state (i.e. via
  /// `clone`)
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError>;
}
