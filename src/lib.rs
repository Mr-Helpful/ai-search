#![warn(missing_docs)]
//! ## Introduction
//!
//! This is a WIP package that implements some pretty common AI search methods.
//! I've mostly created as I've commonly found I need some of these algorithms in other projects
//! I've been working on.
//!
//! ## Algorithms
//!
//! ### Uninformed Search
//!
//! - [X] Breadth First Search
//! - [X] Depth First Search
//! - [X] Iterative Deepening Search
//!
//! ### Informed Search
//!
//! - [X] Uniform Cost Search
//! - [X] Greedy Best First Search
//! - [X] A* Search
//!
//! ### Games
//!
//! - [ ] Minimax Search
//! - [ ] Alpha Beta Pruning

pub mod games;
pub mod prelude;
pub mod search;
pub mod state;
pub mod traits;
pub mod value;
pub mod wrappers;
