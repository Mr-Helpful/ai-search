use std::{fmt::Debug, marker::PhantomData};

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
  fn observe(&self) -> Result<Self::Observation, Self::ObserveError>;

  /// The error produced whilst attempting to transition to a new state
  type ResultError;
  /// Takes an action and transitions into a new state<br>
  /// This should produce a new state from a reference to this state (i.e. via
  /// `clone`)
  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError>;
}

impl<S, Err, Obs, Actn, ObsErr, ResErr, ObsFn, ResFn> State
  for (S, &ResFn, &ObsFn, PhantomData<(Err, Actn, ResErr)>)
where
  Err: Debug + From<ObsErr> + From<ResErr>,
  ObsFn: Fn(&S) -> Result<Obs, ObsErr>,
  ResFn: Fn(&S, &Actn) -> Result<S, ResErr>,
{
  type Error = Err;
  type Observation = Obs;
  type Action = Actn;
  type ObserveError = ObsErr;
  type ResultError = ResErr;

  fn observe(&self) -> Result<Self::Observation, Self::ObserveError> {
    (self.2)(&self.0)
  }

  fn result(&self, action: &Self::Action) -> Result<Self, Self::ResultError> {
    Ok(((self.1)(&self.0, action)?, self.1, self.2, PhantomData))
  }
}

mod depth;
pub use depth::DepthState;
