/*
  RPS! 
  Use Enums for moves
  Rock = A = X = 1
  Paper = B = Y = 2
  Scissors = C = Z = 3
  Win = 6
  Draw = 3
  Loss = 0
*/

use std::collections::HashMap;
use std::fs;

static MOVE_VALUES : HashMap::from([
  ("A", 1),
  ("X", 1),
  ("B", 2),
  ("Y", 2),
  ("C", 3),
  ("Z", 3),
]);

fn Result(opp_val: u32, my_val: u32) -> u32 {
  if opp_val > my_val return 6;
  if opp_val == my_val return 3;
  return 0;
}

fn playRPS(opp_move: str, my_move: str) -> u32 {
  let opp_value = MOVE_VALUES[opp_move];
  let my_value = MOVE_VALUES[my_move];

  return my_value + Result(opp_value, my_value)
}

fn main() {
  let rps_games = fs.read_to_string("data/day2_input.txt").parse();
  println!("Inputs! {}", rps_games)
}
