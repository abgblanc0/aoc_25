use std::collections::HashMap;

fn find_path<'a>(
  devices: &HashMap<&str, Vec<&'a str>>,
  current_device: &'a str,
  memo: &mut HashMap<(String, (bool, bool)), u64>,
  fft_dac: (bool, bool)
) -> u64 {
  if current_device == "out" { 
    return if fft_dac.0 && fft_dac.1 { 1 } else { 0 };
  }

  let paths = devices.get(current_device).unwrap();

  let key = (current_device.to_string(), fft_dac.clone());
  if let Some(p) = memo.get(&key) {
    return *p;
  }
  
  let mut count = 0;
  for path in paths {
    let fft = fft_dac.0 || *path == "fft";
    let dac = fft_dac.1 || *path == "dac";
    count += find_path(devices, path, memo, (fft, dac));
  }

  memo.insert(key, count);
  count
}

pub fn solution(input: &str) {
  let mut devices: HashMap<&str, Vec<&str>> = HashMap::new();
  let mut memo: HashMap<(String, (bool, bool)), u64> = HashMap::new();

  input.lines().for_each(|line| {
    let (origen, destinies) = line.split_once(':').unwrap();

    devices.insert(
      origen,
      destinies.trim().split(' ').collect()
    );
  });

  let result = find_path(&devices, "svr", &mut memo, (false, false));

  println!("Day 11 - Part 1 Solution: {result}");
}
