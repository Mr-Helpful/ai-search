extern crate ai_search;
use ai_search::prelude::*;
use itertools::Itertools;

mod puzzle;
use puzzle::{Direction, TilePuzzle};

const N_ACTIONS: usize = 30;

fn main() {
  println!("Tile Puzzle Example ({} shuffles)", N_ACTIONS);
  let puzzle = TilePuzzle::shuffled(N_ACTIONS);
  println!("{}", puzzle);
  let state = HistoryState::from(GraphState::from(puzzle));
  println!("{}", state);

  let mut search = Astar::new(
    state,
    |_: &Direction| 1,
    |state: &TilePuzzle| {
      (0..4)
        .cartesian_product(0..4)
        .map(|(i, j)| {
          let tile = state[(i, j)];
          let (x, y) = ((tile % 4) as isize, (tile / 4) as isize);
          (x - j as isize).abs() + (y - i as isize).abs()
        })
        .sum::<isize>()
    },
  );

  let state = search.next_goal(TilePuzzle::is_solved).unwrap();
  println!("{}", state);
  println!("{:?}", state.history());
}
