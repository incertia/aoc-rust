use crate::Solution;
use crate::specialize;

// user facing
type ParserFn<'a, I> = fn(input: &'a [u8]) -> I;
type SolverFn<'a, I> = fn(input: &'a I) -> Solution;

// type erased
type ParserInternal = fn(input: &[u8]) -> ();
type SolverInternal = fn(input: &()) -> Solution;
type RunnerInternal =
  unsafe fn(&[u8], &str, fn(&[u8]) -> (), Option<SolverInternal>, Option<SolverInternal>) -> ();

#[derive(Clone, Copy, Debug, Default, Hash)]
pub struct AdventSolver {
  day: i64,
  name: Option<&'static str>,
  p: Option<(ParserInternal, RunnerInternal)>,
  a: Option<SolverInternal>,
  b: Option<SolverInternal>,
}

impl AdventSolver {
  const fn new_empty(day: i64) -> Self {
    Self {
      day,
      name: None,
      p: None,
      a: None,
      b: None,
    }
  }

  pub const fn new<'a, I>(
    day: i64,
    name: Option<&'static str>,
    p: Option<ParserFn<'a, I>>,
    a: Option<SolverFn<'a, I>>,
    b: Option<SolverFn<'a, I>>,
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
        name,
        p: Some((unsafe { core::mem::transmute(p) }, specialize::run_day::<I>)),
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
      let prefix = self.name.unwrap_or("unnamed");

      // SAFETY: these were all cast from the same input type I in
      // AdventSolver::new()
      unsafe { r(input, prefix, p, self.a, self.b) };
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
    let solver = AdventSolver::new(1, None, Some(parse), Some(solve_a), Some(solve_b));
    solver.run(TEST_STRING);
  }
}
