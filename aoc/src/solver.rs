use crate::Solution;

use core::ptr::NonNull;
use std::time::{Duration, Instant};

type SolverInternal = fn(input: &()) -> Solution;
type ParserFn<I> = fn(input: &[u8]) -> I;
type SolverFn<I> = fn(input: &I) -> Solution;
type RunnerInternal = fn(capture: SolverInternal, input: NonNull<()>) -> Solution;

struct Erased {
  pub ptr: NonNull<()>,
  dropper: fn(p: NonNull<()>) -> (),
}

impl Erased {
  pub fn new<T>(data: Box<T>) -> Self {
    fn dropper<T>(p: NonNull<()>) {
      // SAFETY: this comes from a Box<T>
      let ptr = p.as_ptr() as *mut T;
      unsafe {
        // make a new box then drop it
        drop(Box::from_raw(ptr));
      }
    }
    let ptr = Box::into_raw(data);
    // type erase
    // SAFETY: Box::into_raw returns non-null
    let ptr = unsafe { NonNull::new_unchecked(ptr as *mut ()) };
    Self {
      ptr,
      dropper: dropper::<T>,
    }
  }
}

impl Drop for Erased {
  fn drop(&mut self) {
    (self.dropper)(self.ptr);
  }
}

fn run<I>(capture: SolverInternal, input: NonNull<()>) -> Solution {
  // SAFETY: these were transmuted from fn(&I) -> Solution and NonNull<I>
  let f: SolverFn<I> = unsafe { core::mem::transmute(capture) };
  let input: NonNull<I> = unsafe { core::mem::transmute(input) };
  f(unsafe { input.as_ref() })
}

#[derive(Clone, Copy, Debug, Hash)]
struct AdventParser {
  capture: fn(input: &[u8]) -> (),
  parse_erased_fn: fn(&Self, input: &[u8]) -> (Erased, Duration),
  parse_bench_fn: fn(&Self, input: &[u8]) -> Duration,
}

impl AdventParser {
  const fn new<T>(f: fn(&[u8]) -> T) -> Self {
    Self {
      capture: unsafe { core::mem::transmute(f) },
      parse_erased_fn: Self::parse_erased_fn::<T>,
      parse_bench_fn: Self::parse_bench_fn::<T>,
    }
  }

  pub fn parse_erased(&self, input: &[u8]) -> (Erased, Duration) {
    (self.parse_erased_fn)(self, input)
  }

  pub fn parse_bench(&self, input: &[u8]) -> Duration {
    (self.parse_bench_fn)(self, input)
  }

  fn parse_erased_fn<T>(self: &AdventParser, input: &[u8]) -> (Erased, Duration) {
    // SAFETY: we were originally given a fn(&[u8]) -> T
    let f: fn(&[u8]) -> T = unsafe { core::mem::transmute(self.capture) };
    let start = Instant::now();
    let t = f(input);
    let time = start.elapsed();
    (Erased::new(Box::new(t)), time)
  }

  fn parse_bench_fn<T>(self: &AdventParser, input: &[u8]) -> Duration {
    // SAFETY: we were originally given a fn(&[u8]) -> T
    let f: fn(&[u8]) -> T = unsafe { core::mem::transmute(self.capture) };
    core::hint::black_box((|| {
      let start = Instant::now();
      f(input);
      start.elapsed()
    })())
  }
}

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct AdventSolver {
  day: i64,
  a: Option<SolverInternal>,
  b: Option<SolverInternal>,
  p: Option<(AdventParser, RunnerInternal)>,
}

impl AdventSolver {
  pub const fn new_empty(day: i64) -> Self {
    Self {
      day,
      p: None,
      a: None,
      b: None,
    }
  }

  pub const fn new<I>(
    day: i64,
    p: Option<ParserFn<I>>,
    a: Option<SolverFn<I>>,
    b: Option<SolverFn<I>>,
  ) -> Self {
    if let Some(p) = p {
      let ta = match a {
        None => None,
        Some(f) => unsafe { Some(core::mem::transmute(f)) },
      };
      let tb = match b {
        None => None,
        Some(f) => unsafe { Some(core::mem::transmute(f)) },
      };
      Self {
        day,
        p: Some((AdventParser::new(p), run::<I>)),
        a: ta,
        b: tb,
      }
    } else {
      Self::new_empty(day)
    }
  }

  pub const fn day(&self) -> i64 {
    self.day
  }

  pub fn run(&self, input: &[u8]) {
    if let Some((p, r)) = self.p {
      let (input, input_time) = p.parse_erased(input);
      println!("parse: {}us", input_time.as_micros());

      let start = Instant::now();
      let a = self.a.map(|f| r(f, input.ptr));
      let a_time = start.elapsed();

      let start = Instant::now();
      let b = self.b.map(|f| r(f, input.ptr));
      let b_time = start.elapsed();

      match a {
        Some(solution) => println!("a ({}us): {}", a_time.as_micros(), solution),
        _ => (),
      };
      match b {
        Some(solution) => println!("b ({}us): {}", b_time.as_micros(), solution),
        _ => (),
      };

      // input is of type Erased, and gets dropped, which calls the correct
      // freeing function
    }
  }
}

#[cfg(test)]
mod test {
  use super::{AdventSolver, Solution};
  type Input = Vec<u8>;
  const TEST_STRING: &[u8] = b"012345678901234567890123456789012345678901234567890123456789";

  fn solve_a(i: &Vec<u8>) -> Solution {
    Solution::Number(i.len() as i64)
  }
  fn solve_b(i: &Vec<u8>) -> Solution {
    Solution::Text(str::from_utf8(&i[3..10]).expect("ok").to_owned())
  }
  fn parse(i: &[u8]) -> Input {
    i.to_owned()
  }

  #[test]
  fn test_erase() {
    let solver = AdventSolver::new(1, Some(parse), Some(solve_a), Some(solve_b));
    solver.run(TEST_STRING);
  }
}
