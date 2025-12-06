pub fn solution(input: &str) {
  let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
  let mut ops: Vec<char> = lines.pop().unwrap();
  let mut result = 0;

  let mut nums: Vec<Vec<u64>> = Vec::new();
  let mut aux: Vec<u64> = Vec::new();

  for (j, _) in lines[0].iter().enumerate() {
    let mut number: String = String::new();

    for i in 0..lines.len() {
      let n= lines[i][j];
      if !n.is_digit(10) { continue; }
      number.push(n);
    }

    if !number.is_empty() {
      aux.push(number.trim().parse().unwrap());
    }
    if number.is_empty() || j == lines[0].len() - 1 {
      nums.push(aux.clone());
      aux.clear();
    }
  }
  
  ops = ops.into_iter()
    .filter(|op| *op == '+' || *op == '*')
    .collect();

  for (i, op) in ops.iter().enumerate() {
    match op {
      '+' => {
        result += nums[i].iter().sum::<u64>();
      },
      '*' => {
        result += nums[i].iter().product::<u64>();
      },
      _ => continue
    }
  }

  println!("Day 06 - Part 2 Solution: {result}");
}
