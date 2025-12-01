pub fn solution(input: &str) {
  let input: Vec<&str> = input.lines().collect();
  let mut point = 50;
  let mut ceros = 0;
  for line in input {
    let line = line.trim();
    if line.is_empty() { continue; }
    let (dir, dist) = line.split_at(1);
    let dist = dist.parse::<i32>().unwrap();
    match dir {
      "L" => point -= dist,
      "R" => point += dist,
      _ => unreachable!(),
    }
    point = point.rem_euclid(100);
    if point == 0 {
      ceros += 1;
    }
  }
  println!("Day 01 - Part 1 Solution: {ceros}");
}