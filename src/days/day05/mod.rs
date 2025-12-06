mod part1;
mod part2;

use std::fs;

pub fn solution() {
  let input = fs::read_to_string("src/days/day05/input.txt").unwrap().replace("\r\n", "\n");
  part1::solution(&input);
  part2::solution(&input);
}
