mod solvers;

use aoc::aoc_main;

fn main() -> Result<(), aoc::AdventRuntimeError> {
  aoc_main(&solvers::SOLVER)
}
