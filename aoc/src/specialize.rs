use crate::Solution;
use brunch::{Bench, Benches};
use core::ptr::NonNull;
use std::time::{Duration, Instant};

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
  input: &[u8],
  prefix: &str,
  parser: fn(&[u8]) -> (),
  solve_a: Option<fn(&()) -> Solution>,
  solve_b: Option<fn(&()) -> Solution>,
  samples: Option<u32>,
  benches: &mut Benches,
) {
  let my_parse: fn(&[u8]) -> T = unsafe { core::mem::transmute(parser) };
  let my_input = my_parse(input);
  let ptr_input = NonNull::from_ref(&my_input);
  let erased_input = unsafe { core::mem::transmute(ptr_input) };
  let samples = samples.unwrap_or(5000);

  benches.push(
    Bench::new(format!("{}: parse", prefix))
      .with_samples(samples)
      .run(|| my_parse(input)),
  );

  if let Some(solve_a) = solve_a {
    benches.push(
      Bench::new(format!("{}: solve_a", prefix))
        .with_samples(samples)
        .run(|| run::<T>(erased_input, solve_a)),
    );
  }

  if let Some(solve_b) = solve_b {
    benches.push(
      Bench::new(format!("{}: solve_b", prefix))
        .with_samples(samples)
        .run(|| run::<T>(erased_input, solve_b)),
    );
  }
}
