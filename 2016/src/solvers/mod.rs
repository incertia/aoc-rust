use aoc::AdventYearSolver;

mod day01;
mod day02;

pub const SOLVER: AdventYearSolver = AdventYearSolver::new(
  2016,
  &[day01::SOLVER_ITER, day01::SOLVER_COLLECT, day02::SOLVER],
);
