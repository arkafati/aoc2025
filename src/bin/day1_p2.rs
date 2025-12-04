use aoc::{read_input, start, end};

fn handle_left_rot_overflow (location: &mut i32) {
  let initial_loc = *location;
  if initial_loc < 0 {
    *location += (if initial_loc % 100 == 0 { (initial_loc / 100).abs() } else { (initial_loc / 100).abs() + 1 }) * 100;
  }
}

fn handle_left_rotation (rotation: &i32, location: &mut i32) -> i32 {
  let offset = if *location == 0 { -100 } else { 0 - *location };
  let mut total_rotations: i32 = 0;

  if *rotation <= offset {
    total_rotations = 1 + ((rotation - offset) / 100).abs();
  }

  *location += rotation;
  handle_left_rot_overflow(location);

  return total_rotations;
}

fn handle_right_rot_overflow (location: &mut i32) {
  let initial_loc = *location;
  if initial_loc > 99 {
    *location -= (if initial_loc % 100 == 0 { (initial_loc / 100).abs() } else { (initial_loc / 100).abs() }) * 100;
  }
}

fn handle_right_rotation (rotation: &i32, location: &mut i32) -> i32 {
  let offset = if *location == 0 { 100 } else { 100 - *location };
  let mut total_rotations: i32 = 0;

  if *rotation >= offset {
    total_rotations = 1 + ((rotation - offset) / 100).abs();
  }

  *location += rotation;
  handle_right_rot_overflow(location);

  return total_rotations;
}

fn get_rotation(instruction: &str) -> i32 {
  let direction: &str = &instruction[..1];
  let magnitude: i32 = instruction[1..].trim().parse().unwrap();

  let mut multiplier: i8 = 1;

  if direction == "L" {
    multiplier = -1i8;
  }

  return (multiplier as i32) * magnitude;
}

fn main() {
  start();
  let input = read_input(1);
  let mut dial_location: i32 = 50; // Dial starts at 50
  let mut password: i32 = 0;

  for combination in input.lines() {
    let rotation: i32 = get_rotation(combination);

    if rotation < 0 { password += handle_left_rotation(&rotation, &mut dial_location) } 
    else { password += handle_right_rotation(&rotation, &mut dial_location) }

  }

  println!("{}", password);

  end();
}