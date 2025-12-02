pub fn solution(input: &str) {
  let ranges: Vec<(u64, u64)> = input
    .trim()
    .split(',')
    .map(|range| {
      let (start, end) = range
        .trim()
        .split_once('-')
        .expect("Cada rango debe tener un '-'");
      let start = start.parse().expect("Inicio inválido");
      let end = end.parse().expect("Fin inválido");

      (start, end)
    })
    .collect();

  let mut invalids: u64 = 0;

  for range in ranges {
    for i in range.0..=range.1 {
      let i_s = i.to_string();
      if i_s.len() % 2 != 0 { continue; }
      let half = i_s.len() / 2;

      if i_s[0..half] == i_s[half..] {
        invalids += i;
      }
    }
  }

  println!("Day 02 - Part 1 Solution: {}", invalids);
}
