extern crate ai_search;
use ai_search::prelude::*;

mod problem;
use problem::SimplePuzzle;

const N_ACTIONS: usize = 50;

fn main() {
  println!("Simple Puzzle Example ({} shuffles)", N_ACTIONS);
  let puzzle = SimplePuzzle::new(N_ACTIONS);
  println!("{}", puzzle);
  let state = HistoryState::from(GraphState::from(puzzle));
  println!("{}", state);

  let mut search = Bfs::new(state);

  let state = search.next_goal(SimplePuzzle::is_solved).unwrap();
  println!("{}", state);
}
