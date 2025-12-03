fn first_and_second(numbers: &Vec<u32>) -> (u32, u32) {
  let mut first: (u32, usize) = (0, 0);
  let mut second: (u32, usize) = (0, 0);
  (first, second) = numbers.iter().enumerate().fold((first, second), |(f, s), (i, &n)| {
    if n > f.0 { return ((n, i), f); }
    if n > s.0 { return (f, (n, i)); }
    (f, s)
  });

  if first.1 < numbers.len() - 1 { return (first.0, *numbers[first.1 + 1 ..].iter().max().unwrap() )}
  if first.1 > second.1 { return (second.0, first.0); }
  (first.0, second.0)
}

pub fn solution(input: &str) {
  let input: Vec<Vec<u32>> = input.lines().map(|line| {
    line.chars().map(|c| c.to_digit(10).unwrap()).collect()
  }).collect();
  let mut result = 0;

  for numbers in input {
    let (first, second) = first_and_second(&numbers);
    result += first*10 + second;
  }

  println!("Day 03 - Part 1 Solution: {result}");
}