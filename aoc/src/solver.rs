use crate::{AocParser, Solution};

use core::ptr::NonNull;
use std::time::Instant;

type SolverInternal = fn(input: &()) -> Solution;
type SolverFn<I> = fn(input: &I) -> Solution;
type ParseInternal = fn(input: &[u8]) -> Erased;
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

fn erased_parser<I: AocParser>(input: &[u8]) -> Erased {
  Erased::new(Box::new(I::parse(input)))
}

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct AdventSolver {
  day: i64,
  a: Option<SolverInternal>,
  b: Option<SolverInternal>,
  p: Option<(ParseInternal, RunnerInternal)>,
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

  pub const fn new<I: AocParser>(day: i64, a: Option<SolverFn<I>>, b: Option<SolverFn<I>>) -> Self {
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
      p: Some((erased_parser::<I>, run::<I>)),
      a: ta,
      b: tb,
    }
  }

  pub const fn day(&self) -> i64 {
    self.day
  }

  pub fn run(&self, input: &[u8]) {
    if let Some((p, r)) = self.p {
      let input = p(input);

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
