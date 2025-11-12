mod error;
pub mod macros;
mod solver;
pub(crate) mod specialize;
mod year;

use clap::Parser;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Solution {
  Number(i64),
  Text(String),
}

impl std::fmt::Display for Solution {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Solution::Number(i) => write!(f, "{}", i),
      Solution::Text(s) => write!(f, "{}", s),
    }
  }
}

pub use error::AdventRuntimeError;
pub use solver::AdventSolver;
pub use year::AdventYearSolver;

#[derive(Parser)]
struct AocArgs {
  day: Option<i64>,

  #[arg(short, long)]
  bench: bool,

  #[arg(short, long)]
  samples: Option<u32>,
}

pub fn aoc_main(solver: &AdventYearSolver) -> Result<(), AdventRuntimeError> {
  let args = AocArgs::parse();
  let day = args.day.unwrap_or(1);
  if args.bench {
    let mut benches = brunch::Benches::default();
    solver.bench(day, args.samples, &mut benches)?;
    benches.finish();
    Ok(())
  } else {
    solver.run(day)
  }
}
