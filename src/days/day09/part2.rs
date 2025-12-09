use std::collections::HashSet;
use std::fmt;
use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Hash)]
struct Point {
  x: i64,
  y: i64
}

impl Ord for Point {
  fn cmp(&self, other: &Point) -> Ordering {
    self.x.cmp(&other.x).then(self.y.cmp(&other.y))
  }
}

impl fmt::Display for Point {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!(fmt, "({}, {})", self.x, self.y);
    Ok(())
  }
}

fn get_area(point_a: &Point, point_b: &Point) -> i64 {
  let side_a = (point_a.x - point_b.x).abs();
  let side_b = (point_a.y - point_b.y).abs(); 
  (side_a + 1) * (side_b + 1)
}

fn range_between(p1: Point, p2: Point) -> Vec<Point> {
  if p1.x == p2.x {
    let start = p1.y.min(p2.y);
    let end = p1.y.max(p2.y);
    (start..=end).map(|y| Point { x: p1.x, y }).collect()
  } else if p1.y == p2.y {
    let start = p1.x.min(p2.x);
    let end = p1.x.max(p2.x);
    (start..=end).map(|x| Point { x, y: p1.y }).collect()
  } else {
    vec![p1, p2]
  }
}

fn check_side(side: Vec<Point>, sides: &HashSet<Point>, inside: &mut HashSet<Point>) -> bool {
  
  for point in side.iter() {
    let mut ins: bool = false;
    let mut colitions: i32 = 0;

    if inside.contains(point) { continue; }

    for i in (0..=point.y).rev() {
      let p = Point { x: point.x, y: i};

      if sides.contains(&p) && !ins {
        colitions += 1;
        ins = true;
      }
      if !sides.contains(&p) {
        ins = false;
      }
    }
    if colitions % 2 == 1 {
      inside.insert(*point);
    }
    else {
      return false;
    }
  }
  true
}

fn draw_poligon(points: &Vec<Point>) -> (HashSet<Point>, HashSet<Point>) {
  let mut poligon: HashSet<Point> = HashSet::new();
  let mut sides: HashSet<Point> = HashSet::new();

  let a = points.first().unwrap();
  let b = points.last().unwrap();
  let point_a = if a > b { b } else { a };
  let point_b = if b > a { b } else { a };

  for x in point_a.x..point_b.x {
    for y in point_a.y..point_b.y {
      poligon.insert(Point { x, y });
      sides.insert(Point { x, y });
    }
  }

  points.windows(2).for_each(|window| {
    let point_a = if window[0] > window[1] { window[1] } else { window[0] };
    let point_b = if window[1] > window[0] { window[1] } else { window[0] };

    for x in point_a.x..=point_b.x {
      for y in point_a.y..=point_b.y {
        poligon.insert(Point { x, y });
        sides.insert(Point { x, y });
      }
    }
  });

  (poligon, sides)
}

pub fn solution(input: &str) {
  let points: Vec<Point> = input.lines().map(|line| {
    let (a, b) = line.split_once(',').unwrap();
    Point { x: a.parse().unwrap(), y: b.parse().unwrap() }
  }).collect();

  let (mut inside, sides) = draw_poligon(&points);
  let mut areas: Vec<((Point, Point), i64)> = Vec::new();

  
  for (i, point_a) in points.iter().enumerate() {
    for point_b in points[i + 1 ..].iter() {
      if 
        check_side(
          range_between(*point_a, Point { x: point_b.x, y: point_a.y }),
          &sides,
          &mut inside
        ) &&
        check_side(
          range_between(*point_a, Point { x: point_a.x, y: point_b.y }),
          &sides,
          &mut inside
        ) &&
        check_side(
          range_between(*point_b, Point { x: point_a.x, y: point_b.y }),
          &sides,
          &mut inside
        ) &&
        check_side(
          range_between(*point_b, Point { x: point_b.x, y: point_a.y }),
          &sides,
          &mut inside
        ) 
      {
        areas.push(((*point_a, *point_b), get_area(point_a, point_b)));
      }
    }
  }
  
  areas.sort_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b).reverse());
  println!("Day 09 - Part 1 Solution: {}", areas.first().unwrap().1);
}
