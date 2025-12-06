pub fn solution(input: &str) {
  let (all_ids, available_ids) = input.split_once("\n\n").unwrap();

  let all_ids: Vec<(u64, u64)> = all_ids.lines().map(|line| {
    let (first, last): (u64, u64) = line.split_once("-").map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap())).unwrap();
    (first, last)
  }).collect();
  let available_ids: Vec<u64> = available_ids.lines().map(|line| line.parse().unwrap()).collect();

  let mut result = 0;

  for id in available_ids {
    if all_ids.iter().any(|(first, last)| id >= *first && id <= *last) {
      result += 1;
    }
  }
  println!("Day 05 - Part 1: {result}");
}
