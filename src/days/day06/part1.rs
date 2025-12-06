use std::vec;

use regex::Regex;

pub fn solution(input: &str) {
  let mut nums: Vec<Vec<u64>> = Vec::new();
  let mut ops: Vec<char> = Vec::new();
  let mut result = 0;

  for (index, line) in input.lines().enumerate() {
    let re = Regex::new(r" {2,}").unwrap();
    let line = re.replace_all(line.trim(), " ").to_string();

    for (j, num) in line.split(" ").enumerate() {
      if index == input.lines().count() - 1 {
        ops.push(num.chars().next().unwrap());
      }
      else if index == 0 {
        nums.push(vec![num.parse().unwrap()]);
      }
      else { 
        nums[j].push(num.parse().unwrap());
      }
    }
  }

  for (i, op) in ops.iter().enumerate() {
    match op {
      '+' => {
        result += nums[i].iter().sum::<u64>();
      },
      '*' => {
        result += nums[i].iter().product::<u64>();
      },
      _ => unreachable!()
    }
  }
  println!("Day 06 - Part 1 Solution: {result}");
}
