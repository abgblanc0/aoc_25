fn get_area(gift: &str) -> u32 {
  gift.lines().map(|line| {
    line.chars().filter(|c| *c == '#').count() as u32
  }).sum()
}

pub fn solution(input: &str) {
  let mut input: Vec<&str> = input.split("\n\n").collect();
  let trees: Vec<&str> = input.pop().unwrap().lines().collect();
  let gifts: Vec<u32> = input.iter().map(|gift| get_area(gift)).collect();

  let mut result = 0;

  for tree in trees {
    let (w_h, gs) = tree.split_once(":").unwrap();
    let (w, h) = w_h.split_once("x").unwrap();
    let area = w.parse::<u32>().unwrap() * h.parse::<u32>().unwrap();
    let mut area_gifts = 0;

    for (i, gift) in gs.trim().split(" ").enumerate() {
      let cuant_gift = gift.parse::<u32>().unwrap();

      area_gifts += cuant_gift * gifts[i];  
    }

    result += if area_gifts <= area { 1 } else { 0 };
  }
  println!("Day 12 - Part 1 Solution: {result}");
}
