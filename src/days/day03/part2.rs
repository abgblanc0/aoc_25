fn batteries(numbers: &[u32]) -> Vec<u32> {
  let n = numbers.len();
  if n <= 12 { return numbers.to_vec(); }

  let mut result = Vec::new();
  let mut pos = 0;

  for i in 0..12 {
    let remaining_needed = 12 - i;
    let max_possible_end = n - remaining_needed;

    let mut best_val = 0;
    let mut best_idx = pos;
    for j in pos..=max_possible_end {
      if numbers[j] > best_val {
        best_val = numbers[j];
        best_idx = j;
      }
    }

    result.push(best_val);
    pos = best_idx + 1;
  }

  result
}


pub fn solution(input: &str) {
  let input: Vec<Vec<u32>> = input.lines().map(|line| {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
  }).collect();
  let mut result: u64 = 0;

  for numbers in input {
    result += batteries(&numbers).iter().fold(0_u64, |acc, &digit| acc * 10 + digit as u64);
  }

  println!("Day 03 - Part 2 Solution: {result}");
}
