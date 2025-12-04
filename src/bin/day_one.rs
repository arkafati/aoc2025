use aoc::read_input;

pub fn track_password (location: i32, password: &mut u32) {
  if location == 0 {
    *password += 1;
  }
}

pub fn handle_left_rot_overflow (location: &mut i32) {
  let initial_loc = *location;
  *location += if initial_loc % 100 == 0 { (initial_loc / 100).abs() * 100 } else { ((initial_loc / 100).abs() + 1) * 100 };
}

pub fn handle_right_rot_overflow (location: &mut i32) {
  let initial_loc = *location;
  *location -= if initial_loc % 100 == 0 { (initial_loc / 100).abs() * 100 } else { ((initial_loc / 100).abs() - 1) * 100 };
}

pub fn get_rotation(instruction: &str) -> i32 {
  let direction: &str = &instruction[..1];
  let magnitude: i32 = instruction[1..].trim().parse().unwrap();

  let mut multiplier: i8 = 1;

  if direction == "L" {
    multiplier = -1i8;
  }

  return (multiplier as i32) * magnitude;

}

pub fn main() -> () {
  let input: String = read_input(1);
  let mut dial_location: i32 = 50; // Dial starts at 50
  let mut password: u32 = 0;

  for combination in input.lines() {
    let rotation: i32 = get_rotation(combination);
    dial_location += rotation;

    if dial_location < 0 {
      handle_left_rot_overflow(&mut dial_location);
    } 
    
    if dial_location > 99 {
      handle_right_rot_overflow(&mut dial_location);
    }

    track_password(dial_location, &mut password);
  }

  println!("{}", password);
}