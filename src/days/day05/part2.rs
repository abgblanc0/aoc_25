pub fn solution(input: &str) {
  let (all_ids, _) = input.split_once("\n\n").unwrap();
  let all_ids: Vec<(u64, u64)> = all_ids.lines().map(|line| {
    let (first, last): (u64, u64) = line.split_once("-").map(|n| (n.0.parse().unwrap(), n.1.parse().unwrap())).unwrap();
    (first, last)
  }).collect();

  let mut ranges: Vec<(u64, u64)> = all_ids;
  ranges.sort_by_key(|r| r.0);

  let mut merged = vec![];
  for range in ranges {
    if merged.is_empty() {
      merged.push(range);
    } else {
      let last = merged.last_mut().unwrap();
      if last.1 + 1 >= range.0 {
        last.1 = last.1.max(range.1);
      } else {
        merged.push(range);
      }
    }
  }

  let result: u64 = merged.iter().map(|(a, b)| b - a + 1).sum();
  println!("Day 05 - Part 2: {result}");
}
