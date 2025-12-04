use aoc::{start, end, read_input};

fn largest_two_digit_in_order(s: &str) -> u32 {
  let digits: Vec<u32> = s.chars()
    .filter_map(|c| c.to_digit(10))
    .collect();

  let mut max_val = 0;
  let mut largest_so_far = digits[0];

  for &d in &digits[1..] {
    let candidate = largest_so_far * 10 + d;
    if candidate > max_val {
      max_val = candidate;
    }
    if d > largest_so_far {
      largest_so_far = d;
    }
  }

  return max_val;
}

pub fn main() {
  start();

  let input = read_input(3);

  let mut sum = 0;

  for line in input.lines() {
    let max = largest_two_digit_in_order(line);
    sum += max;
  }

  println!("{}", sum);

  end();
}