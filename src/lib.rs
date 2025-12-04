use std::fs;
use std::time::Instant;

static mut START_TIME: Option<Instant> = None;

pub fn start() {
  unsafe {
    START_TIME = Some(Instant::now());
  }
}

pub fn end() {
  unsafe {
    match START_TIME {
      Some(start_time) => {
        let elapsed = start_time.elapsed();
        println!("Operation complete after: {:.3?}", elapsed);
      },
      None => {}
    }
  }
}

pub fn read_input(day: u8) -> String {
  return fs::read_to_string(format!("./input/day_{}.txt", day)).expect("Failed to load input");
}