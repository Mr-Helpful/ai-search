use std::fmt::Debug;

/// A generic implementation of state for search methods.
///
/// We seperate out getting all applicable actions and picking a specific
/// action here to allow search methods to keep track of the actions taken so
/// far and limit the actions that can be taken.
///
/// States can be thought of as similar to nodes on a graph, with the actions
/// representing edges all with a weight of 1. We also look to solve the
/// slightly reduced problem of finding any path to a goal state, rather than
/// the shortest path.
pub trait State: Sized {
  /// A supertype for all errors thrown during state generation<br>
  /// All errors should be convertable into this type
  type Error: Debug + From<Self::ObserveError> + From<Self::ResultError>;

  /// A type for observations of the state<br>
  /// This should be a (potentially) restricted view on the state
  type Observation;

  /// A type for actions that can be taken on this state<br>
  /// We should be able to transition from this state with anything satisfying
  /// the action type
  type Action;

  /// The error produced whilst attempting to observe the state
  type ObserveError;
  /// Returns an observation of the state<br>
  /// This should be used with to determine the next action to take on the state
  fn observe(&self) -> Result<&Self::Observation, Self::ObserveError>;

  /// The error produced whilst attempting to transition to a new state
  type ResultError;
  /// Takes an action and transitions into a new state<br>
  /// This should produce a new state from a reference to this state (i.e. via
  /// `clone`)
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError>;
}
