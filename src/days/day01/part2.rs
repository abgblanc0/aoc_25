pub fn solution(input: &str) {
  let input: Vec<&str> = input.lines().collect();
  let mut point = 50;
  let mut ceros = 0;

  for line in input {
    let line = line.trim();
    if line.is_empty() { continue; }
    let (dir, dist) = line.split_at(1);
    let dist = dist.parse::<i32>().unwrap();
    let step = if dir == "R" { 1 } else { -1 };

    for _ in 0..dist {
      point = (point + step as i32).rem_euclid(100);
      if point == 0 {
        ceros += 1;
      }
    }
  }
  println!("Day 01 - Part 2 Solution: {ceros}");
}