use std::{collections::HashMap, fmt};

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Point {
  x: i64,
  y: i64,
  z: i64
}

const MAX_PAIRS: i32 = 1000;

impl fmt::Display for Point {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    let _ = write!(fmt, "({}, {}, {})", self.x, self.y, self.z);
    Ok(())
  }
}

fn dist(point_a: &Point, point_b: &Point) -> i64 {
  ((point_a.x - point_b.x).pow(2) + (point_a.y - point_b.y).pow(2) + (point_a.z - point_b.z).pow(2)).isqrt()
}

pub fn solution(input: &str) {
  let points: Vec<Point> = input.lines().map(|line| {
    let mut aux = line.split(',');
    let (x, y, z) = (aux.next().unwrap(), aux.next().unwrap(), aux.next().unwrap());
    Point{ x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
  }).collect();
  let mut distancies: Vec<((Point, Point), i64)> = Vec::new();
  let mut connections: Vec<Vec<Point>> = Vec::new();
  let mut connections_id: HashMap<Point, usize> = HashMap::new();
  let mut next_id = 0;

  for (i, point_a) in points.iter().enumerate() {
    for point_b in points[i + 1 ..].iter() {
      if point_a == point_b { continue; }
      distancies.push(((*point_a, *point_b), dist(point_a, point_b)));
    }
  }

  distancies.sort_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b));
  distancies = distancies[0..MAX_PAIRS as usize].to_vec();

  for pair in distancies.iter() {
    let (point_a, point_b) = pair.0;

    let id1 = connections_id.get(&point_a);
    let id2 = connections_id.get(&point_b);

    match (id1, id2) {
      (Some(&id1), Some(&id2)) if id1 == id2 => continue,
      (Some(&id1), Some(&id2)) => {
        let (low, high) = if id1 < id2 { (id1, id2) } else { (id2, id1) };

        let (left, right) = connections.split_at_mut(high);
        let comp_low = &mut left[low];
        let comp_high = &mut right[0];

        for &p in comp_high.iter() {
          connections_id.insert(p, low);
        }
        comp_low.append(comp_high);
      }
      (Some(&id), None) => {
        connections[id].push(point_b);
        connections_id.insert(point_b, id);
      }
      (None, Some(&id)) => {
        connections[id].push(point_a);
        connections_id.insert(point_a, id);
      }
      (None, None) => {
        let new_id = next_id;
        next_id += 1;
        connections.push(vec![point_a, point_b]);
        connections_id.insert(point_a, new_id);
        connections_id.insert(point_b, new_id);
      }
    }
  }


  connections.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
  connections.retain(|conn| !conn.is_empty());

  let mut result: u64 = 1;
  
  for i in 0..3 {
    result *= connections[i].len() as u64;
  }
  println!("Day 08 - Part 1 Solution: {result}");
}
