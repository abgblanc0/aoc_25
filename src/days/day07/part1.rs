pub fn get_char(matrix: &Vec<Vec<char>>, index: (usize, usize)) -> char {
  *matrix.get(index.0).unwrap_or(&vec!['-']).get(index.1).unwrap_or(&'-')
}

pub fn beam(pos: (usize, usize), matrix: &mut Vec<Vec<char>>) -> u32 {
  let mut pos = pos;
  let mut count = 0;
  let mut c = get_char(&matrix, pos);

  while c == '.' {
    matrix[pos.0][pos.1] = '|';
    pos = (pos.0 + 1, pos.1);
    c = get_char(&matrix, pos);
  }

  if c == '^' {
    count = 1;
    if get_char(&matrix, (pos.0, pos.1 - 1)) == '.' {
      count += beam((pos.0, pos.1 - 1), matrix);
    }
    if get_char(&matrix, (pos.0, pos.1 + 1)) == '.' {
      count += beam((pos.0, pos.1 + 1), matrix);
    }
  }
  count
}

pub fn solution(input: &str) {
  let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let pos_s: (usize, usize) = (0, matrix[0].len() / 2);
  let result = beam((pos_s.0 + 1, pos_s.1), &mut matrix);

  //matrix.iter().for_each(|row| println!("{}", row.iter().collect::<String>()));
  println!("Day 07 - Part 1 Solution: {result}");
}
