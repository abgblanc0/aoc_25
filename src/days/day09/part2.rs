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

/* 
fn check_side(range: (Point, Point)) -> bool {


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
  true
}

*/
fn draw_poligon(points: &Vec<Point>, matrix: &mut Vec<Vec<char>>) {

  let a = points.first().unwrap();
  let b = points.last().unwrap();

  for x in a.x..=b.x {
    for y in a.y..=b.y {
      matrix[y as usize][x as usize] = 'X';
    }
  }

  points.windows(2).for_each(|window| {
    let point_a = if window[0] > window[1] { window[1] } else { window[0] };
    let point_b = if window[1] > window[0] { window[1] } else { window[0] };

    for x in point_a.x..=point_b.x {
      for y in point_a.y..=point_b.y {
        matrix[y as usize][x as usize] = 'X';
      }
    }
  });
}

pub fn solution(input: &str) {
  let mut max_x = i64::MIN;
  let mut max_y = i64::MIN;

  let points: Vec<Point> = input.lines().map(|line| {
    let (a, b) = line.split_once(',').unwrap();
    let x = a.parse().unwrap();
    let y = b.parse().unwrap();
    max_x = max_x.max(x);
    max_y = max_y.max(y);
    Point { x: a.parse().unwrap(), y: b.parse().unwrap() }
  }).collect();

  let mut matrix: Vec<Vec<char>> = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];

  draw_poligon(&points, &mut matrix);

  matrix.iter().for_each(|row| println!("{}", row.iter().collect::<String>()));

}
