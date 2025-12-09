#[derive(PartialEq, Clone, Copy)]
struct Point {
  x: i64,
  y: i64
}

fn get_area(point_a: &Point, point_b: &Point) -> i64 {
  let side_a = (point_a.x - point_b.x).abs();
  let side_b = (point_a.y - point_b.y).abs(); 
  (side_a + 1) * (side_b + 1)
}

pub fn solution(input: &str) {
  let points: Vec<Point> = input.lines().map(|line| {
    let (a, b) = line.split_once(',').unwrap();
    Point { x: a.parse().unwrap(), y: b.parse().unwrap() }
  }).collect();
  let mut areas: Vec<((Point, Point), i64)> = Vec::new();

  for (i, point_a) in points.iter().enumerate() {
    for point_b in points[i + 1 ..].iter() {
      if point_a == point_b { continue; }
      areas.push(((*point_a, *point_b), get_area(point_a, point_b)));
    }
  }
  areas.sort_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b).reverse());

  let area = areas.first().unwrap();
  println!("Day 09 - Part 1 Solution: {}", area.1);
}
