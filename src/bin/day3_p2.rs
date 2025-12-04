use aoc::{start, end, read_input};

fn largest_two_digit_in_order(s: &str) -> u64 {
  let digits: Vec<u32> = s.chars()
    .filter_map(|c| c.to_digit(10))
    .collect();

  const BATTERY_COUNT: usize = 12;
  let mut stack: Vec<u32> = Vec::new();
  let mut len_ref = digits.len();


  for &d in &digits {
    // Try to shift the digit as far up the stack as possible
    while !stack.is_empty() && stack.last().unwrap() < &d && stack.len() + len_ref - 1 >= BATTERY_COUNT {
        stack.pop();
    }
    if stack.len() < BATTERY_COUNT { stack.push(d) }
    len_ref -= 1;
  }

  let mut max: u64 = 0;
  for d in stack {
    max = max * 10 + d as u64
  }

  return max;
}

pub fn main() {
  start();

  let input = read_input(3);

  let mut sum: u64= 0;

  for line in input.lines() {
    let max = largest_two_digit_in_order(line);
    sum += max;
  }

  println!("{}", sum);

  end();
}