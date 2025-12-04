fn get_char(matrix: &Vec<Vec<char>>, index: (usize, usize)) -> char {
  *matrix.get(index.0).unwrap_or(&vec!['.']).get(index.1).unwrap_or(&'.')
}
fn check_around(pos: (usize, usize), matrix: &mut Vec<Vec<char>>) -> bool {
  let mut rolls = 0;
  let directions: [(i32, i32); 8] = [
    (-1, 0),
    (0, -1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (0, 1),
    (1, 0),
    (1, 1),
  ];

  for (dx, dy) in directions {
    let x = pos.0 as i32 + dx;
    let y = pos.1 as i32 + dy;
    let c = get_char(matrix, (x as usize, y as usize));

    if c == '@' || c == 'X' {
      rolls += 1;
    }

    if rolls > 3 {
      return false;
    }
  }
  
  matrix[pos.0][pos.1] = '.';
  true
}

pub fn solution(input: &str) {
  let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut removed = 0;
  let mut changed = true;

  while changed {
    changed = false;
    let mut i = 0;
    while i < matrix.len() {
      let mut j = 0;
      while j < matrix[i].len() {
        if matrix[i][j] == '@' && check_around((i, j), &mut matrix) {
          removed += 1;
          changed = true;
        }
        j += 1;
      }
      i += 1;
    }
  }

  println!("Day 04 - Part 2 Solution: {removed}");
}