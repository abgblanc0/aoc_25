use std::collections::HashMap;

pub fn get_char(matrix: &Vec<Vec<char>>, index: (usize, usize)) -> char {
  matrix
    .get(index.0)
    .and_then(|row| row.get(index.1))
    .copied()
    .unwrap_or('-')
}

pub fn beam(pos: (usize, usize), matrix: &Vec<Vec<char>>, memo: &mut HashMap<(usize, usize), u64>) -> u64 {

  if memo.contains_key(&pos) {
    return *memo.get(&pos).unwrap();
  }

  let mut p = pos;
  let mut c = get_char(matrix, pos);
  let mut count = 0u64;

  while c == '.' {
    p = (p.0 + 1, p.1);
    c = get_char(matrix, p);
  }

  if c == '-' {
    count += 1;
  }

  if c == '^' {
    let left = get_char(matrix, (p.0, p.1 - 1));
    let right = get_char(matrix, (p.0, p.1 + 1));
    if left == '.' {
      count += beam((p.0, p.1 - 1), matrix, memo);
    }
    if right == '.'  {
      count += beam((p.0, p.1 + 1), matrix, memo);
    }
  }

  memo.insert(pos, count);
  count
}

pub fn solution(input: &str) {
  let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let pos_s: (usize, usize) = (0, matrix[0].len() / 2);
  let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
  let count = beam((pos_s.0 + 1, pos_s.1), &matrix, &mut memo);

  println!("Day 07 - Part 2 Solution: {count}");
}

