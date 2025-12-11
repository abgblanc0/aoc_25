use std::collections::HashMap;

fn find_path<'a>(
  devices: &HashMap<&str, Vec<&'a str>>,
  current_device: &'a str,
  memo: &mut HashMap<&'a str, u32>,
) -> u32 {
  if current_device == "out" { return 1; }
  let paths = devices.get(current_device).unwrap();
  let mut count = 0;

  if let Some(p) = memo.get(current_device) {
    return *p;
  }

  for path in paths {
    count += find_path(devices, path, memo);

  }
  memo.insert( current_device, count);
  count
}

pub fn solution(input: &str) {
  let mut devices: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut memo: HashMap<&str, u32> = HashMap::new();

  input.lines().for_each(|line| {
    let (origen, destinies) = line.split_once(':').unwrap();

    devices.insert(
      origen,
      destinies.trim().split(' ').collect()
    );
  });

  let result = find_path(&devices, "you", &mut memo);

  println!("Day 11 - Part 1 Solution: {result}");
}
