fn divisors(n: u64) -> Vec<u64> {
  let mut result = Vec::new();
  for i in 1..n {
    if n % i == 0 { result.push(i); }
  }
  result
}

fn split_number(n: u64, parts: usize) -> Vec<u64> {
  let s = n.to_string();

  s.as_bytes()
    .chunks(parts)
    .map(|chunk| {
      std::str::from_utf8(chunk).unwrap().parse::<u64>().unwrap()
    })
    .collect()
}


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
      let len = i.to_string().len();
      let divs = divisors(len as u64);
      
      for div in divs {
        let parts = split_number(i, div as usize);
        if parts.iter().all(|&part| *parts.first().unwrap() == part) {
          invalids += i;
          break;
        }
      }
    }
  }

  println!("Day 02 - Part 2 Solution: {invalids}");
}
