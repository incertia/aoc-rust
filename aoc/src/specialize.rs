use crate::Solution;
use core::ptr::NonNull;
use std::time::{Duration, Instant};

// fn(&[u8]) -> T =>
pub fn run<T>(input: NonNull<()>, r: fn(&()) -> Solution) -> (Solution, Duration) {
  let input_t: NonNull<T> = unsafe { core::mem::transmute(input) };
  let my_ref: &T = unsafe { input_t.as_ref() };
  let my_runner: fn(&T) -> Solution = unsafe { core::mem::transmute(r) };

  let start = Instant::now();
  let solution = core::hint::black_box(my_runner(my_ref));
  let time = start.elapsed();
  (solution, time)
}

// SAFETY: the caller must guarantee that the functions parser, solve_a, and
// solve_b were cast/transmuted from fuctions of the correct type. namely,
// fn(&[u8]) -> T and fn(&T) -> Solution
pub unsafe fn run_day<T>(
  input: &[u8],
  prefix: &str,
  parser: fn(&[u8]) -> (),
  solve_a: Option<fn(&()) -> Solution>,
  solve_b: Option<fn(&()) -> Solution>,
) {
  let my_parse: fn(&[u8]) -> T = unsafe { core::mem::transmute(parser) };
  let start = Instant::now();
  let my_input = core::hint::black_box(my_parse(input));
  let time = start.elapsed();
  println!("{}: parse: {}us", prefix, time.as_micros());

  let ptr_input = NonNull::from_ref(&my_input);
  let input = unsafe { core::mem::transmute(ptr_input) };

  let a = solve_a.map(|f| run::<T>(input, f));
  let b = solve_b.map(|f| run::<T>(input, f));
  let print = |s: (Solution, Duration), part: &str| {
    println!("{}: {} ({}us): {}", prefix, part, s.1.as_micros(), s.0);
  };

  a.into_iter().for_each(|s| print(s, "a"));
  b.into_iter().for_each(|s| print(s, "b"));
}

pub unsafe fn bench_day<T>(
  _input: &[u8],
  _prefix: &str,
  _parser: fn(&[u8]) -> (),
  _solve_a: Option<fn(&()) -> Solution>,
  _solve_b: Option<fn(&()) -> Solution>,
) {
  todo!()
}
