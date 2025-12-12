mod part1;
mod part2;

use std::fs;

pub fn solution() {
  let input: String = fs::read_to_string("src/days/day12/input.txt").unwrap().replace("\r\n", "\n");
  part1::solution(&input);
  part2::solution(&input);
}
